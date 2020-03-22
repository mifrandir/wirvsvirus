use super::*;
use rand;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

#[derive(Debug)]
pub struct Society {
    config: Arc<Config>,
    cities: Vec<Arc<Mutex<City>>>,
    //city_relations: Vec<Vec<u8>>,
    deaths: u32,
    recoveries: u32,
    infections: u32,
    new_infections: u32,
}

#[derive(Debug)]
pub enum Event {
    Death,
    Recovery,
    Infection,
}

enum InternalEvent {
    Infection(usize),
    Encounter(usize, usize),
}

const THREAD_NUMBER: u8 = 7;

impl Society {
    pub fn new(cfg: Config) -> Self {
        // TODO: Refactor.
        let begin = time::Instant::now();
        let cities = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        let size = cfg.population.size;
        let city_no = (size / cfg.population.city_size) as usize;
        eprintln!("{}", city_no);
        let (sq, rq) = mpsc::channel();
        for i in 0..city_no {
            sq.send(i).unwrap();
        }
        drop(sq);
        let receiver = Arc::new(Mutex::new(rq));
        let cfg = Arc::new(cfg);
        for _ in 0..THREAD_NUMBER {
            let cities = Arc::clone(&cities);
            let receiver = Arc::clone(&receiver);
            let cfg = Arc::clone(&cfg);
            handles.push(thread::spawn(move || loop {
                let inst = (*receiver).lock().unwrap().recv();
                if let Err(_) = inst {
                    return;
                }
                let city_mut = Mutex::new(City::new(Arc::clone(&cfg)));
                (*cities).lock().unwrap().push(city_mut);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        eprintln!("Built all cities in {}s", begin.elapsed().as_secs_f32());
        let mut cities_unlocked = Vec::new();
        let it = Arc::try_unwrap(cities).unwrap().into_inner().unwrap();
        for c in it.into_iter() {
            cities_unlocked.push(Arc::new(c));
        }
        // TODO: Reintroduce city relations.
        //let begin2 = time::Instant::now();
        //let mut city_relations = Vec::new();
        //for _ in 0..city_no {
        //    city_relations.push(vec![0; city_no as usize]);
        //}
        //eprintln!(
        //    "Built city relations placeholder in {}s",
        //    begin2.elapsed().as_secs_f32()
        //);
        //let begin2 = time::Instant::now();
        //let mut rng = rand::thread_rng();
        //for i in 1..city_no {
        //    for j in 0..i {
        //        let r: u8 = rng.gen();
        //        city_relations[i][j] = r;
        //        city_relations[j][i] = r;
        //    }
        //}
        //eprintln!(
        //    "Built city relations in {}s",
        //    begin2.elapsed().as_secs_f32()
        //);
        eprintln!("Built society in {}s", begin.elapsed().as_secs_f32());
        Society {
            new_infections: 0,
            config: cfg,
            cities: cities_unlocked,
            //city_relations: city_relations,
            deaths: 0,
            infections: 0,
            recoveries: 0,
        }
    }
    pub fn init(&mut self) {
        (*self.cities)[0].lock().unwrap().infect(0);
        self.infections += 1;
    }
    pub fn next_day(&mut self) {
        let before = self.infections;
        self.calculate_internal_change();
        self.calculate_national_infections();
        self.new_infections = self.infections - before;
    }

    fn calculate_internal_change(&mut self) {
        let (sx, rx) = mpsc::channel();
        let mut handles = Vec::new();
        for c in self.cities.iter() {
            let c = Arc::clone(c);
            let sx = sx.clone();
            handles.push(thread::spawn(move || {
                let mut lock = (*c).lock().unwrap();
                (*lock).next_day(Some(&sx));
                (*lock).calculate_infections(Some(&sx));
                (*lock).update_hosptials(Some(&sx));
            }));
        }
        drop(sx);
        self.handle_events(rx);
    }

    fn calculate_national_infections(&mut self) {
        let mut per_city = Vec::new();
        for c in self.cities.iter() {
            let city = (*c).lock().unwrap();
            per_city.push((*city).get_national_mobile());
        }
        let mut all_con_c = Vec::new();
        for (i, ps) in per_city.iter().enumerate() {
            let mut con_c = 0;
            let city = self.cities[i].lock().unwrap();
            for p in ps {
                if (*city).people[*p].is_contagious(&self.config) {
                    con_c += 1;
                }
            }
            all_con_c.push(con_c);
        }
        let mut rev_con_c = vec![0; all_con_c.len()];
        for (i, con_c) in all_con_c.iter().enumerate() {
            for j in 0..rev_con_c.len() {
                if j != i {
                    rev_con_c[j] += con_c;
                }
            }
        }
        let mut rng = rand::thread_rng();
        let (sx, rx) = mpsc::channel();
        let total = per_city.iter().map(|x| x.len()).sum::<usize>() as f32;
        for (i, c) in self.cities.iter().enumerate() {
            let (ev_s, ev_r) = mpsc::channel();
            let mut city = (*c).lock().unwrap();
            let p = rev_con_c[i] as f32 / (total - per_city[i].len() as f32);
            for j in 0..per_city[i].len() {
                if rng.gen::<f32>() < p {
                    ev_s.send(InternalEvent::Infection(per_city[i][j])).unwrap();
                }
            }
            drop(ev_s);
            (*city).handle_internal_events(ev_r, Some(&sx));
        }
        drop(sx);
        self.handle_events(rx);
    }

    fn handle_events(&mut self, rx: mpsc::Receiver<Event>) {
        for e in rx {
            match e {
                Event::Infection => self.infections += 1,
                Event::Death => self.deaths += 1,
                Event::Recovery => self.recoveries += 1,
            }
        }
    }
    pub fn active(&self) -> u32 {
        self.infections - self.deaths - self.recoveries
    }
    pub fn to_string(&self) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            self.infections,
            self.deaths,
            self.recoveries,
            self.active(),
            self.new_infections,
            self.in_treatment(),
            self.in_treatmet_queue(),
        )
    }
    pub fn csv_header(&self) -> String {
        format!("Infections,Deaths,Recoveries,Active,New Infections,Hospital,Hospital Queue")
    }

    fn in_treatment(&self) -> usize {
        let mut total = 0;
        for c in self.cities.iter() {
            let lock = (*c).lock().unwrap();
            total += (*lock).hospital.len();
        }
        total
    }

    fn in_treatmet_queue(&self) -> usize {
        let mut total = 0;
        for c in self.cities.iter() {
            let lock = (*c).lock().unwrap();
            total += (*lock).hospital_queue.len();
        }
        total
    }
}

#[derive(Debug)]
pub struct City {
    infections: u32,
    dead: u32,
    recovered: u32,
    active: u32,
    people: Vec<Person>,
    people_relations: HashMap<(u32, u32), u8>,
    //district_relations: Vec<Vec<u8>>,
    //household_relations: Vec<Vec<u8>>,
    config: Arc<Config>,
    hospital: VecDeque<usize>,
    hospital_queue: VecDeque<usize>,
}
impl City {
    pub fn new(cfg: Arc<Config>) -> Self {
        let begin = time::Instant::now();
        let size = cfg.population.city_size;
        let mut people = Vec::new();
        let mut age: i8 = -1;
        let mut left = 0;
        for _ in 0..size as i32 {
            if left == 0 {
                if age < 9 {
                    age += 1;
                }
                left = (cfg.population.age_distribution[age as usize] * size as f32) as i32;
            }
            people.push(Person::new(age));
            left -= 1;
        }
        people.shuffle(&mut rand::thread_rng());
        let mut people_relations = HashMap::new();
        let h_size = cfg.population.household_size;
        let h_no = size / h_size;
        for h in 0..h_no {
            for i in 1..h_size {
                for j in 0..i {
                    let r = rand::random();
                    people_relations.insert((h * h_size + i, h * h_size + j), r);
                    people_relations.insert((h * h_size + j, h * h_size + i), r);
                }
            }
        }
        // TODO: Reintroduce those relations.
        //let mut household_relations = Vec::new();
        //let h_no = h_no as usize;
        //for _ in 0..h_no {
        //    household_relations.push(vec![0; h_no]);
        //}
        //for i in 1..h_no as usize {
        //    for j in 0..i {
        //        let r = rand::random();
        //        household_relations[i][j] = r;
        //        household_relations[j][i] = r;
        //    }
        //}
        //let mut district_relations = Vec::new();
        //let d_size = cfg.population.district_size;
        //let d_no = (size / d_size) as usize;
        //for _ in 0..d_no {
        //    district_relations.push(vec![0; d_no]);
        //}
        //for i in 1..d_no as usize {
        //    for j in 0..i {
        //        let r = rand::random();
        //        district_relations[i][j] = r;
        //        district_relations[j][i] = r;
        //    }
        //}
        eprintln!("Built city in {}s", begin.elapsed().as_secs_f64());
        City {
            infections: 0,
            active: 0,
            dead: 0,
            recovered: 0,
            people: people,
            people_relations: people_relations,
            //district_relations: district_relations,
            //household_relations: household_relations,
            config: cfg,
            hospital: VecDeque::new(),
            hospital_queue: VecDeque::new(),
        }
    }

    fn get_national_mobile(&self) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut mobile = Vec::new();
        for i in 0..self.people.len() {
            // We don't allow people to leave the city if they are in quarantine...
            if self.people[i].in_treatment {
                continue;
            }
            if rng.gen::<f32>() < self.config.population.mean_national_mobility {
                mobile.push(i);
            }
        }
        mobile
    }

    fn infect(&mut self, a: u32) {
        if a > self.config.population.city_size {
            panic!("Given index exceeds city size.");
        }
        self.people[a as usize].infect(&*self.config, None);
    }

    fn handle_internal_events(
        &mut self,
        ev_r: mpsc::Receiver<InternalEvent>,
        sx: Option<&mpsc::Sender<Event>>,
    ) {
        for e in ev_r {
            match e {
                InternalEvent::Encounter(a, b) => self.handle_encounter(a, b, sx),
                InternalEvent::Infection(a) => self.handle_infection(a, sx),
            }
        }
    }

    fn calculate_infections(&mut self, sx: Option<&mpsc::Sender<Event>>) {
        // We're using such a channel in case we want to make this concurrent later on.
        let (ev_s, ev_r) = mpsc::channel();
        self.calculate_household_infections(&ev_s);
        self.calculate_district_infections(&ev_s);
        self.calculate_city_infections(&ev_s);
        drop(ev_s);
        self.handle_internal_events(ev_r, sx);
    }

    fn handle_infection(&mut self, a: usize, sx: Option<&mpsc::Sender<Event>>) {
        if self.people[a].infect(&self.config, sx) {
            //if self.infections == 0 {
            //    eprintln!("First infected in new city");
            //}
            self.infections += 1;
            self.hospital_queue.push_back(a);
        }
    }

    fn handle_encounter(&mut self, a: usize, b: usize, sx: Option<&mpsc::Sender<Event>>) {
        if !self.is_relevant_encounter(a, b) {
            return;
        }
        let res;
        match self.people_relations.get(&(a as u32, b as u32)) {
            Some(v) => res = *v,
            None => panic!("Relation between {} and {} does not exist!", a, b),
        }
        let r: u8 = rand::thread_rng().gen();
        if r < res {
            // In households we're doing 1:1 infections.
            // This is unlike the model in the bigger partitions where we're using pools.
            self.handle_infection(a, sx);
            self.handle_infection(b, sx);
        }
    }
    fn calculate_household_infections(&mut self, ev_q: &mpsc::Sender<InternalEvent>) {
        let mut rng = rand::thread_rng();
        let h_size = self.config.population.household_size as usize;
        let h_no = self.config.population.city_size as usize / h_size;
        for h in 0..h_no {
            let mut is_mobile = Vec::new();
            for i in 0..h_size {
                let real_mobility = self.config.population.mean_household_mobility * {
                    if self.people[h * h_size + i].in_treatment {
                        1.0 - self.config.virus.treatment_quarantine_efficiency
                    } else {
                        1.0
                    }
                };
                let m = real_mobility > rng.gen();
                is_mobile.push(m);
                if !m {
                    continue;
                }
                for j in 0..i {
                    if !is_mobile[j] {
                        continue;
                    }
                    let a = h * h_size + i;
                    let b = h * h_size + j;
                    ev_q.send(InternalEvent::Encounter(a, b)).unwrap();
                }
            }
        }
    }

    fn calculate_district_infections(&mut self, ev_q: &mpsc::Sender<InternalEvent>) {
        let mut rng = rand::thread_rng();
        let d_size = self.config.population.district_size as usize;
        let d_no = self.config.population.city_size as usize / d_size;
        for d in 0..d_no {
            let mut total_mobile = Vec::new();
            for i in 0..d_size {
                let real_mobility = self.config.population.mean_district_mobility * {
                    if self.people[d * d_size + i].in_treatment {
                        1.0 - self.config.virus.treatment_quarantine_efficiency
                    } else {
                        1.0
                    }
                };
                if real_mobility <= rng.gen() {
                    continue;
                }
                total_mobile.push(i);
            }
            for i in total_mobile.iter() {
                let h_size = self.config.population.household_size as usize;
                let h = *i / h_size;
                let foreign_mobile: Vec<usize> = total_mobile
                    .iter()
                    .map(|x| *x)
                    .filter(|x| h != *x / h_size)
                    .collect();
                let total = foreign_mobile.len();
                let mut con_c = 0;
                for x in foreign_mobile {
                    if self.people[x].is_contagious(&self.config) {
                        con_c += 1;
                    }
                }
                let p = con_c as f32 / total as f32;
                if p < rng.gen() {
                    continue;
                }
                ev_q.send(InternalEvent::Infection(d * d_size + *i))
                    .unwrap();
            }
        }
    }

    fn calculate_city_infections(&mut self, ev_q: &mpsc::Sender<InternalEvent>) {
        let mut rng = rand::thread_rng();
        let c_size = self.config.population.city_size as usize;
        let mut total_mobile = Vec::new();
        for i in 0..c_size {
            let real_mobility = self.config.population.mean_city_mobility * {
                if self.people[i].in_treatment {
                    1.0 - self.config.virus.treatment_quarantine_efficiency
                } else {
                    1.0
                }
            };
            if real_mobility <= rng.gen() {
                continue;
            }
            total_mobile.push(i);
        }
        for i in total_mobile.iter() {
            let d_size = self.config.population.district_size as usize;
            let d = *i / d_size;
            let foreign_mobile: Vec<usize> = total_mobile
                .iter()
                .map(|x| *x)
                .filter(|x| d != *x / d_size)
                .collect();
            let total = foreign_mobile.len();
            let mut con_c = 0;
            for x in foreign_mobile {
                if self.people[x].is_contagious(&self.config) {
                    con_c += 1;
                }
            }
            let p = con_c as f32 / total as f32;
            if p < rng.gen() {
                continue;
            }
            ev_q.send(InternalEvent::Infection(*i)).unwrap();
        }
    }

    fn update_hosptials(&mut self, _sx: Option<&mpsc::Sender<Event>>) {
        while let Some(p) = self.hospital.front() {
            if self.people[*p].recovered || self.people[*p].dead {
                self.hospital.pop_front();
            } else {
                break;
            }
        }
        while self.hospital.len() < self.config.population.city_medical_capacity {
            if let Some(p) = self.hospital_queue.pop_front() {
                self.hospital.push_back(p);
                self.people[p].hospitalize(&self.config);
            } else {
                break;
            }
        }
    }

    fn is_relevant_encounter(&self, a: usize, b: usize) -> bool {
        let pa = &self.people[a];
        let pb = &self.people[b];
        if pb.dead || pa.dead {
            return false;
        }
        if pa.infected == pb.infected {
            return false;
        }
        if pa.is_contagious(&*self.config) == pb.is_contagious(&*self.config) {
            return false;
        }
        return true;
    }

    fn next_day(&mut self, sx: Option<&mpsc::Sender<Event>>) {
        for i in 0..self.people.len() {
            self.people[i].next_day(&self.config, sx);
        }
    }
}

#[derive(Debug)]
pub struct Person {
    age: i8,
    infected: bool,
    infected_for: Option<i32>,
    recovered: bool,
    doomed: Option<bool>,
    severity: u8,
    dead: bool,
    in_treatment: bool,
    in_treatment_for: Option<i32>,
}

impl Person {
    pub fn new(mut age: i8) -> Self {
        // We only consider 90+, not 100+, 110+, etc.
        if age > 9 {
            age = 9;
        }
        Person {
            age: age,
            infected: false,
            infected_for: None,
            recovered: false,
            doomed: None,
            severity: 0,
            dead: false,
            in_treatment: false,
            in_treatment_for: None,
        }
    }

    fn recover(&mut self, sx: Option<&mpsc::Sender<Event>>) {
        if let Some(s) = sx {
            s.send(Event::Recovery).unwrap();
        }
        self.recovered = true;
    }

    fn kill(&mut self, sx: Option<&mpsc::Sender<Event>>) {
        if let Some(s) = sx {
            s.send(Event::Death).unwrap();
        }
        self.dead = true;
    }

    fn infect(&mut self, cfg: &Config, sx: Option<&mpsc::Sender<Event>>) -> bool {
        if self.infected {
            return false;
        }
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() >= cfg.virus.contagiousness {
            return false;
        }
        if let Some(s) = sx {
            s.send(Event::Infection).unwrap();
        }
        self.infected = true;
        self.infected_for = Some(0);
        self.severity = rng.gen();
        true
    }

    fn hospitalize(&mut self, _cfg: &Config) {
        self.in_treatment = true;
        self.in_treatment_for = Some(0);
    }

    fn is_doomed(&mut self, cfg: &Config) -> bool {
        if let Some(b) = self.doomed {
            return b;
        }
        let t = if let Some(it) = self.in_treatment_for {
            let inf = self.infected_for.unwrap() as f32;
            let delay = inf - it as f32;
            1.0 - cfg.virus.treatment_efficiency * (1.0 - cfg.virus.treatment_decay.powf(-delay))
        } else {
            1.0
        };
        let real_lethality = cfg.virus.lethality[self.age as usize] * t;
        let b = self.severity as f32 > std::u8::MAX as f32 * (1.0 - real_lethality);
        self.doomed = Some(b);
        b
    }

    pub fn next_day(&mut self, cfg: &Config, sx: Option<&mpsc::Sender<Event>>) {
        let inf;
        match self.infected_for {
            None => return,
            Some(t) => inf = t + 1,
        }
        self.infected_for = Some(inf);
        if let Some(t) = self.in_treatment_for {
            self.in_treatment_for = Some(t + 1);
        }
        if self.recovered || self.dead {
            return;
        }
        if inf > cfg.virus.sick_for {
            if self.is_doomed(cfg) {
                self.kill(sx);
            } else {
                //eprintln!(
                //    "Recovered with severity {} ({}%)",
                //    self.severity,
                //    100.0 * self.severity as f32 / std::u8::MAX as f32
                //);
                self.recover(sx);
            }
        }
    }

    pub fn is_contagious(&self, cfg: &Config) -> bool {
        if let Some(time) = self.infected_for {
            time < cfg.virus.contagious_for
        } else {
            false
        }
    }
}
