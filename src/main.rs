use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;

extern crate rand;
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;

type Kysyttava = HashMap<String, String>;


fn main() {

    println!("Luetaan tiedostosta kyseltävät sanat...");
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
    /*for b in &a {
        println!("{:?}", b.get("perusmuoto"));

    }*/

    kysele(&a);
    
}

/// Sisältää varsinaisen kyselysilmukan
fn kysele(sanalista: &Vec<Kysyttava>) {
    // Luetaan argumentit talteen ja tulkitaan siitä kysyttävät asiat.
    let args: Vec<String> = env::args().collect();
    let mut vihjemuoto = "perusmuoto";
    let mut vastausmuoto = "te-muoto";
    if args.len() >= 3 {
        vihjemuoto = &args[1];
        vastausmuoto = &args[2];
    }

    let mut rng = thread_rng();
    println!("Tervetuloa kyselyyn!");
    loop{
        // Etsitään kysyttävä
        let kysyttava = match anna_satunnainen_kysyttava(&mut rng, &sanalista, &vihjemuoto, &vastausmuoto){
            Some(a) => a,
            None => {
                println!("Sadalla yrityksellä ei löytynyt yhtään kysyttävää :(");
                break;
            }
        };
        // Kysytään kysyttävä
        match kysy_sana(kysyttava, &vastausmuoto, &vihjemuoto){
            VastauksenTulos::Poistu => break,
            _ => (),

        }
        println!("");
    }

    println!("Kiitoksia käytöstä. Tervetuloa uudelleen!");

}

fn anna_satunnainen_kysyttava<'a>(rng: &mut ThreadRng, kysymyslista: &'a Vec<Kysyttava>, vihje: &str, vastaus: &str) -> Option<&'a Kysyttava>{
    // Varmistetaan ettei ole ikuinen silmukka, jos oikeanlaista kysyttävää ei edes ole olemassa.
    for _i in 0..100{
        let kysyttava = &kysymyslista.get(rng.gen_range(0, kysymyslista.len())).unwrap();
        let v1 = kysyttava.get(vihje)?;
        let v2 = kysyttava.get(vastaus)?;
        if v1.trim() == "" || v2.trim() == "-" || v1.trim() == "-" || v2.trim() == "" 
        {
            continue;
        }
        return Some(kysyttava);

    }
    None

}

enum VastauksenTulos {
    Oikein,
    Vaarin,
    Poistu,
    Luovuta,
}


/// Kysyy annetusta sanasta halutulla vihjeellä haluttua muotoa.
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
            _ => {
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
