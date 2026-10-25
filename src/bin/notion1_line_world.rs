// =====================================================================
// Notion 1 : la boucle agent / environnement, sur Line World
// =====================================================================
//
// Règles (support Projet, §3.1) :
//   - couloir de n cases, numérotées de 0 à n-1
//   - l'agent part d'une case intérieure `depart`
//   - action 0 = gauche, action 1 = droite (permises seulement hors extrémités)
//   - entrer en case 0     -> récompense -1, fin de l'épisode
//   - entrer en case n-1   -> récompense +1, fin de l'épisode
//   - tout autre mouvement -> récompense 0
//
// Interface imposée par le prof (support Projet, §4.1) :
//   reset()               -> remet l'état initial et le renvoie
//   actions_possibles(s)  -> liste des actions permises dans s (ordre croissant)
//   step(a)               -> (état suivant, récompense, fin)
//
// Résultat attendu avec n = 6, départ 2, actions 1 1 0 1 1 1 :
//   3 0.0000 0
//   4 0.0000 0
//   3 0.0000 0
//   4 0.0000 0
//   5 1.0000 1
// =====================================================================

use std::thread::sleep;

struct LineWorld {
    n: usize,
    depart: usize,
    etat: usize,
}

impl LineWorld {
    fn new(n: usize, depart: usize) -> LineWorld {
        LineWorld {
            n,
            depart,
            etat: depart,
        }
    }

    // reset() :
    //     etat <- depart
    //     renvoyer etat
    fn reset(&mut self) -> usize {
        self.etat = self.depart;
        self.etat
    }

    // actions_possibles(s) :
    //     si s est une extrémité (0 ou n-1) -> renvoyer liste vide (état terminal)
    //     sinon -> renvoyer [0, 1]
    fn actions_possibles(&self, s: usize) -> Vec<usize> {
        if s == 0 || s == self.n - 1 {
            vec![]
        } else {
            vec![0, 1]
        }
    }

    // step(a) :
    //     si a == 0 : etat <- etat - 1
    //     sinon     : etat <- etat + 1
    //
    //     si etat == 0       -> renvoyer (etat, -1.0, vrai)
    //     si etat == n - 1   -> renvoyer (etat, +1.0, vrai)
    //     sinon              -> renvoyer (etat,  0.0, faux)
    fn step(&mut self, a: usize) -> (usize, f64, bool) {
        if a == 0 {
            self.etat = self.etat - 1;
        } else {
            self.etat = self.etat + 1;
        }

        if self.etat == 0 {
            (self.etat, -1.0, true)
        } else if self.etat == self.n - 1 {
            (self.etat, 1.0, true)
        } else {
            (self.etat, 0.0, false)
        }
    }
}

// main :
//     env <- LineWorld::new(6, 2)
//     s <- env.reset()
//     afficher s et env.actions_possibles(s)      (pour vérifier, avec {:?})
//
//     pour chaque a dans [1, 1, 0, 1, 1, 1] :
//         (s2, r, fin) <- env.step(a)
//         afficher "s2 r fin"   avec r sur 4 décimales et fin en 1 / 0
//         si fin -> sortir de la boucle
fn main() {
    let mut env = LineWorld::new(6,2);
    let s =env.reset();

    println!("{}",s);
    println!("{:?}",env.actions_possibles(s));

    for a in [1,1,0,1,1,1] {
        let (s2,r,fin) = env.step(a);
        let fin_affiche = if fin { 1 } else { 0 };
        println!("{} {:.4} {}",s2,r,fin_affiche);
        if fin {
            break;
        }
    }
}

// ---------------------------------------------------------------------
// BONUS (agent humain, demandé par le prof pour tester un environnement) :
// remplace la liste d'actions par une lecture au clavier :
//     let mut ligne = String::new();
//     std::io::stdin().read_line(&mut ligne).unwrap();
//     let a: usize = ligne.trim().parse().unwrap();
// et affiche le couloir à chaque pas, par exemple :  [-1][  ][🤖][  ][  ][+1]
// ---------------------------------------------------------------------
