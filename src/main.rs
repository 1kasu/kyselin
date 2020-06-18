use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

extern crate rand;
use rand::{thread_rng, Rng};

type Kysyttava = HashMap<String, String>;


fn main() {
    println!("Hello, world!");

    // Avataan tiedosto, jos voidaan ja kerätään rivit talteen
    let mut rivit = match File::open("sanalista.lst"){
        Ok(t) => keraa_rivit_tiedostosta(t),
        Err(e) => {
            println!("Annettua listaa ei voitu avata: {}", e.to_string());
            return
        },
    };

    // Parsitaan tiedoston sisältö
    let a = parsi_riveista(&mut rivit).unwrap();
    for b in &a {
        //println!("{:?}", b.get("perusmuoto"));

    }

    kysele(&a);
    
}

fn kysele(sanalista: &Vec<Kysyttava>) {
    let mut rng = thread_rng();
    println!("Tervetuloa kyselyyn!");
    loop{


        match kysy_sana(sanalista.get(rng.gen_range(0, sanalista.len())).unwrap(), "te-muoto", "perusmuoto"){
            VastauksenTulos::Poistu => break,
            _ => (),

        }
        println!("");
    }

    println!("Kiitoksia käytöstä. Tervetuloa uudelleen!");

}

enum VastauksenTulos {
    Oikein,
    Vaarin,
    Poistu,
    Luovuta,
}

fn kysy_sana(kysyttava: &Kysyttava, haluttu_muoto: &str, vihjemuoto: &str) -> VastauksenTulos{
    let oikea_vastaus = kysyttava.get(haluttu_muoto).unwrap();
    let vihje = kysyttava.get(vihjemuoto).unwrap();
    let mut syote = String::new();

    println!("Anna {} sanasta: {}", haluttu_muoto, vihje);

    // Luetaan syöte ja verrataan sitä
    match io::stdin().read_line(&mut syote) {
        Err(e) => println!("Virhe: {}", e),
        Ok(_) => return match syote.trim() {
            "exit" => VastauksenTulos::Poistu,
            "luovuta" => VastauksenTulos::Luovuta,
            vastaus if vastaus == oikea_vastaus => {
                println!("Oikein!"); 
                VastauksenTulos::Oikein
            },
            a => {
                println!("Väärin, oikea vastaus: {}", oikea_vastaus);
                VastauksenTulos::Vaarin
            }

        },
    }
    VastauksenTulos::Poistu
}

/// Muodostaa annetusta tiedostosta lukijan, joka antaa rivin kerrallaan
fn keraa_rivit_tiedostosta(tiedosto: File) -> io::Lines<io::BufReader<File>>
{
    io::BufReader::new(tiedosto).lines()
}


/// Parsii rivit tietorakenteeseen.
fn parsi_riveista(rivit: &mut io::Lines<io::BufReader<File>>)
    -> Result<Vec<Kysyttava>, io::Error> {

    let otsikkorivi:String = match rivit.next(){
        // Karsitaan io virheet pois tässä vaiheessa ekalta riviltä
        Some(a) => a?,
        // Tyhjä tiedosto käytännössä
        None => return Ok(Vec::new()),
    };

    // Heitetään vektoriin, koska jostain syystä paljon helpompaa käyttää uudelleen
    let otsikot: Vec<_> = otsikkorivi.split("|").collect();

    let mut kysyttavat = Vec::new();

    for rivi in rivit.into_iter() {
        let mut kysyttava = Kysyttava::new();

        let apu = rivi?;// Karsii io virheen pois.
        let mut kohdat = apu.split("|").into_iter();

        for otsikko in otsikot.iter() {
            match kohdat.next() {
                Some(kohta) => {
                    //println!("{},{}", otsikko.to_string(), kohta.to_string());
                    kysyttava.insert(otsikko.trim().to_string(),kohta.trim().to_string());
                },
                None => break,
            }
        }

        kysyttavat.push(kysyttava);
    }

    Ok(kysyttavat)
}
