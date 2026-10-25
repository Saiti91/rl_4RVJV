// =====================================================================
// Notion 2 : l'hypothèse de Markov, sur Pierre-feuille-ciseaux
// =====================================================================
//
// Règles (support Projet, §3.3) :
//   - actions : 0 = pierre, 1 = feuille, 2 = ciseaux
//   - feuille bat pierre, ciseaux bat feuille, pierre bat ciseaux
//   - manche 1 : l'adversaire joue floor(3 * tirage)
//   - manche 2 : l'adversaire rejoue le coup de l'agent à la manche 1
//   - +1 gagné, -1 perdu, 0 égalité
//   - état : 0 à la manche 1, puis 1 + a1 à la manche 2, puis 4 (terminal)
//
// L'expérience :
//   pour chaque coup a1 possible à la manche 1, l'agent joue ensuite PIERRE
//   à la manche 2. On compare deux façons de coder l'état de la manche 2 :
//     - l'état markovien du prof : 1 + a1
//     - un état "naïf"           : toujours 1
//   et on regarde si (état, action) suffit à prédire la récompense.
//
// Résultat attendu (tirage = 0.5) :
//   a1=0 | etat markovien=1 | etat naif=1 | pierre -> 0.0000
//   a1=1 | etat markovien=2 | etat naif=1 | pierre -> -1.0000
//   a1=2 | etat markovien=3 | etat naif=1 | pierre -> 1.0000
// =====================================================================

struct PierreFeuilleCiseaux {
    etat: usize,
    adversaire: usize, // le coup que l'adversaire va jouer à la manche en cours
}

// resultat(agent, adv) :
//     si agent == adv               -> 0.0
//     si agent == (adv + 1) mod 3   -> +1.0   (ta règle "coup + 1 modulo 3" !)
//     sinon                         -> -1.0
fn resultat(agent: usize, adv: usize) -> f64 {
    todo!()
}

impl PierreFeuilleCiseaux {
    fn new() -> PierreFeuilleCiseaux {
        PierreFeuilleCiseaux { etat: 0, adversaire: 0 }
    }

    // reset(tirage) :
    //     adversaire <- floor(3 * tirage)      (indice : (3.0 * tirage) as usize)
    //     etat <- 0
    //     renvoyer etat
    fn reset(&mut self, tirage: f64) -> usize {
        todo!()
    }

    // step(a) :
    //     r <- resultat(a, adversaire)
    //     si etat == 0 :                       (manche 1)
    //         adversaire <- a                  (il rejouera notre coup)
    //         etat <- 1 + a
    //         renvoyer (etat, r, faux)
    //     sinon :                              (manche 2)
    //         etat <- 4
    //         renvoyer (etat, r, vrai)
    fn step(&mut self, a: usize) -> (usize, f64, bool) {
        todo!()
    }
}

// main :
//     env <- PierreFeuilleCiseaux::new()
//     pour a1 de 0 à 2 :
//         env.reset(0.5)
//         (s, _, _) <- env.step(a1)          manche 1
//         (_, r, _) <- env.step(0)           manche 2 : pierre
//         afficher a1, s, l'état naïf (1) et r
//
//     Question : avec l'état naïf, la même paire (état 1, pierre) donne
//     trois récompenses différentes. Qu'est-ce que ça veut dire ?
fn main() {
    todo!()
}
