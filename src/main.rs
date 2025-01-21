use std::{fs, path::Path};

use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use colored::*;

use std::io::{self, Write};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NiveauUrgence {
    Faible,
    Moyen,
    Eleve,
    Critique,
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct Traitement{
    medicament: String,
    posologie: String,
    date_debut: String,
    date_fin: Option<String>,
    prescrit_par: u32,
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct NoteMedicale{
    date: String,
    contenu: String,
    auteur: u32,
}



#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct  DossierMedical{
    antecedents :Vec<String>,
    allergies: Vec<String>,
    groupe_sanguin : String,
    traitements :Vec<Traitement>,
    notes : Vec<NoteMedicale>,
}

#[derive(Debug, Serialize,Deserialize,Clone)]
pub struct Patient {
    pub id:u32,
    pub nom:String,
    pub prenom:String,
    pub date_naissance:String,
    pub numero_secu:String,
    pub dossier_medical: DossierMedical,
    pub niveau_urgence: Option<NiveauUrgence>,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Personnel {
    pub id:u32,
    pub nom:String,
    pub prenom:String,
    pub specialite:String,
    pub  status:String,
    // planning: Planning,
    qualifications: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Planning {
    horaires: Vec<Horaire>,
    gardes: Vec<Garde>,
    conges: Vec<Periode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Horaire {
    jour: String,
    debut: String,
    fin: String,
    service: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Garde {
    date: String,
    service: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Periode {
    debut: String,
    fin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Service {
    id: u32,
    nom: String,
    chef_service: u32,
    capacite: u32,
    personnel_affecte: Vec<u32>,
    equipements: Vec<Equipement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Equipement {
    id: u32,
    nom: String,
    statut: StatutEquipement,
    derniere_maintenance: String,
    prochaine_maintenance: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum StatutEquipement {
    Fonctionnel,
    EnMaintenance,
    HorsService,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Medicament {
    id: u32,
    nom: String,
    description: String,
    stock: u32,
    seuil_alerte: u32,
    date_peremption: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Pharmacie {
    medicaments: Vec<Medicament>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Facture {
    id: u32,
    patient_id: u32,
    prestations: Vec<Prestation>,
    total: f64,
    date_emission: String,
    statut: StatutFacture,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Prestation {
    description: String,
    montant: f64,
    code_acte: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum StatutFacture {
    EnAttente,
    Payee,
    Annulee,
}


#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct RendezVous {
    pub id:u32,
    pub date:String,
    pub heure:String,
    pub patient_id :u32,
    pub personnel_id:u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Utilisateur {
    id: u32,
    nom_utilisateur: String,
    mot_de_passe_hash: String,
    role: Role,
    derniere_connexion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Role {
    Admin,
    Medecin,
    Infirmier,
    Secretaire,
}


#[derive(Debug,Deserialize,Serialize)]
pub  struct Application{
    patients: Vec<Patient>,
    personnel: Vec<Personnel>,
    rendez_vous: Vec<RendezVous>,
    services: Vec<Service>,
    pharmacie: Pharmacie,
    factures: Vec<Facture>,
    utilisateurs: Vec<Utilisateur>,

}



impl Application {
    pub fn new() ->Application{
        Application{
            patients:Vec::new(),
            personnel:Vec::new(),
            rendez_vous:Vec::new(),
            services:Vec::new(),
            pharmacie: Pharmacie { medicaments: Vec::new() },
            factures:Vec::new(),
            utilisateurs:Vec::new(),
            
        }
    }

    pub  fn creer_utilisateur(&mut self) {

        println!("{}", "\n=== CRÉATION D'UN NOUVEL UTILISATEUR ===".green());

        let id = (self.utilisateurs.len() + 1) as u32;
        let nom_utilisateur = lire_chaine("Nom Utilisateur: ");
        let mot_de_passe = lire_chaine("Mot de passe: ");
        let mot_de_passe_hash = hash(mot_de_passe.as_bytes(), DEFAULT_COST).unwrap();

        println!("Rôle:");
        println!("1. Admin");
        println!("2. Médecin");
        println!("3. Infirmier");
        println!("4. Secrétaire");
        let role = match lire_nombre("Choix: ") {
            1 => Role::Admin,
            2 => Role::Medecin,
            3 => Role::Infirmier,
            _ => Role::Secretaire,
        };

        let utilisateur = Utilisateur{
            id, nom_utilisateur,mot_de_passe_hash,role,derniere_connexion: None,
        };

        self.utilisateurs.push(utilisateur);
        self.save_data();
        println!("{}", "\nUtilisateur créé avec succès!".green());
        
    }

    pub  fn charger_data() ->Self {
        if Path::new("data.json").exists() {
            let data = fs::read_to_string("data.json").unwrap_or_default();
            serde_json::from_str(&data).unwrap_or(Application::new())
        } else {
            Application::new()
        }
    }

    pub fn save_data(&self){
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write("data.json", data).unwrap();
    }

    pub  fn ajouter_patient(&mut self) {
        println!("{}", "\n=== AJOUT D'UN PATIENT ===".green());
        let id = (self.patients.len() +1) as u32;
        let nom = lire_chaine("Nom: ");
        let prenom = lire_chaine("Prenom: ");
        let date_naissance = lire_chaine("Date de naissance (JJ/MM/AAAA): ");
        let numero_secu = lire_chaine("Numéro de sécurité sociale: ");

        let dossier_medical = DossierMedical {
            antecedents: Vec::new(),
            allergies: Vec::new(),
            groupe_sanguin: String::new(),
            traitements: Vec::new(),
            notes: Vec::new(),
        };

        let patient = Patient{
            id,nom,prenom,date_naissance,numero_secu, dossier_medical, niveau_urgence:None
        };
        self.patients.push(patient);
        self.save_data();
        println!("{}", "\nPatient ajouté avec succès!".green());
    }

    pub fn liste_patients(&self) {
        println!("{}", "\n=== LISTE DES PATIENTS ===".green());

        for  patient in &self.patients  {
            println!("{}", "-".repeat(40));
            println!("ID: {}", patient.id);
            println!("Nom: {} {}", patient.nom, patient.prenom);
            println!("Date de naissance: {}", patient.date_naissance);
            println!("N° Sécu: {}", patient.numero_secu);
        }
    }

    pub   fn ajouter_personnel(&mut self) {
        println!("{}", "\n=== AJOUT DE PERSONNEL ===".green());

        let id = (self.personnel.len() + 1) as u32;
        let nom = lire_chaine("Nom: ");
        let prenom = lire_chaine("prenom: ");
        let specialite = lire_chaine("Spécialité: ");
        let status = String::from("En service");
        let qualifications = Vec::new();

        let personnel = Personnel {
            id,
            nom,
            prenom,
            specialite,
            status,
            qualifications,
        };

        self.personnel.push(personnel);
        self.save_data();
        println!("{}", "\nMembre du personnel ajouté avec succès!".green());
    }

    pub fn liste_personnel(&self) {
        println!("{}", "\n=== LISTE DU PERSONNEL ===".green());
        for pers in &self.personnel {
            println!("{}", "-".repeat(40));
            println!("ID: {}", pers.id);
            println!("Dr. {} {}", pers.nom, pers.prenom);
            println!("Spécialité: {}", pers.specialite);
            println!("Status: {}", pers.status);
        }
    }

    pub fn ajouter_rendez_vous(&mut self) {
        println!("{}", "\n=== NOUVEAU RENDEZ-VOUS ===".green());
        
        let id = (self.rendez_vous.len() + 1) as u32;
        let date = lire_chaine("Date (JJ/MM/AAAA): ");
        let heure = lire_chaine("Heure (HH:MM): ");
        
        self.liste_patients();
        let patient_id = lire_nombre("ID du patient: ");

        self.liste_personnel();
        let personnel_id = lire_nombre("ID du médecin: ");

        let rdv = RendezVous{id,date,heure,patient_id,personnel_id};

        self.rendez_vous.push(rdv);
        self.save_data();
        println!("{}", "\nRendez-vous ajouté avec succès!".green());

    }

    pub fn liste_rendez_vous(&self) {
        println!("{}", "\n=== LISTE DES RENDEZ-VOUS ===".green());
        for rdv in &self.rendez_vous {
            println!("{}", "-".repeat(40));
            let patient = self.patients.iter().find(|p| p.id == rdv.patient_id).unwrap();
            let medecin = self.personnel.iter().find(|p| p.id == rdv.personnel_id).unwrap();
            println!("ID: {}", rdv.id);
            println!("Date: {} à {}", rdv.date, rdv.heure);
            println!("Patient: {} {}", patient.nom, patient.prenom);
            println!("Médecin: Dr. {} {}", medecin.nom, medecin.prenom);
        }
    }

    
    pub fn ajouter_note_medicale(&mut self) {
        self.liste_patients();
        let patient_id = lire_nombre("ID du patient: ");
        
        if let Some(patient) = self.patients.iter_mut().find(|p| p.id == patient_id) {
            println!("\n=== AJOUT D'UNE NOTE MÉDICALE ===");
            let date = chrono::Local::now().format("%d/%m/%Y").to_string();
            let contenu = lire_chaine("Contenu de la note: ");
            let auteur = lire_nombre("ID du médecin: ");

            let note = NoteMedicale {
                date,
                contenu,
                auteur,
            };

            patient.dossier_medical.notes.push(note);
            self.save_data();
            println!("{}", "\nNote médicale ajoutée avec succès!".green());
        } else {
            println!("{}", "Patient non trouvé!".red());
        }
    }

    // Gestion de la pharmacie
    fn ajouter_medicament(&mut self) {
        println!("{}", "\n=== AJOUT D'UN MÉDICAMENT ===".green());
        let id = (self.pharmacie.medicaments.len() + 1) as u32;
        let nom = lire_chaine("Nom du médicament: ");
        let description = lire_chaine("Description: ");
        let stock = lire_nombre("Quantité en stock: ");
        let seuil_alerte = lire_nombre("Seuil d'alerte: ");
        let date_peremption = lire_chaine("Date de péremption (JJ/MM/AAAA): ");

        let medicament = Medicament {
            id,
            nom,
            description,
            stock,
            seuil_alerte,
            date_peremption,
        };

        self.pharmacie.medicaments.push(medicament);
        self.save_data();
        println!("{}", "\nMédicament ajouté avec succès!".green());
    }

    fn verifier_stock(&self) {
        println!("{}", "\n=== ÉTAT DES STOCKS ===".green());
        for med in &self.pharmacie.medicaments {
            println!("{}", "-".repeat(40));
            println!("Médicament: {}", med.nom);
            println!("Stock: {}", med.stock);
            if med.stock <= med.seuil_alerte {
                println!("{}", "⚠️ Stock faible!".red());
            }
            println!("Péremption: {}", med.date_peremption);
        }
    }

    // Gestion des services
    fn ajouter_service(&mut self) {
        println!("{}", "\n=== AJOUT D'UN SERVICE ===".green());
        let id = (self.services.len() + 1) as u32;
        let nom = lire_chaine("Nom du service: ");
        let chef_service = lire_nombre("ID du chef de service: ");
        let capacite = lire_nombre("Capacité d'accueil: ");

        let service = Service {
            id,
            nom,
            chef_service,
            capacite,
            personnel_affecte: Vec::new(),
            equipements: Vec::new(),
        };

        self.services.push(service);
        self.save_data();
        println!("{}", "\nService ajouté avec succès!".green());
    }

    // Gestion des factures
    fn creer_facture(&mut self) {
        println!("{}", "\n=== CRÉATION D'UNE FACTURE ===".green());
        let id = (self.factures.len() + 1) as u32;
        self.liste_patients();
        let patient_id = lire_nombre("ID du patient: ");

        let mut prestations = Vec::new();
        loop {
            println!("\nAjouter une prestation ? (O/N)");
            if lire_chaine("").to_uppercase() != "O" {
                break;
            }

            let description = lire_chaine("Description: ");
            let montant = lire_chaine("Montant: ").parse::<f64>().unwrap_or(0.0);
            let code_acte = lire_chaine("Code acte: ");

            prestations.push(Prestation {
                description,
                montant,
                code_acte,
            });
        }

        let total = prestations.iter().map(|p| p.montant).sum();
        let date_emission = chrono::Local::now().format("%d/%m/%Y").to_string();

        let facture = Facture {
            id,
            patient_id,
            prestations,
            total,
            date_emission,
            statut: StatutFacture::EnAttente,
        };

        self.factures.push(facture);
        self.save_data();
        println!("{}", "\nFacture créée avec succès!".green());
    }



    // Statistiques étendues
    fn afficher_statistiques(&self) {
        println!("{}", "\n=== STATISTIQUES DE L'HÔPITAL ===".green());
        
        // Statistiques générales
        println!("\n--- Statistiques Générales ---");
        println!("Nombre total de patients: {}", self.patients.len());
        println!("Nombre total de personnel: {}", self.personnel.len());
        println!("Nombre de services: {}", self.services.len());
        
        // Statistiques des rendez-vous
        println!("\n--- Rendez-vous ---");
        let rdv_aujourdhui = self.rendez_vous.iter()
            .filter(|r| r.date == chrono::Local::now().format("%d/%m/%Y").to_string())
            .count();
        println!("Rendez-vous aujourd'hui: {}", rdv_aujourdhui);
        
        // Statistiques financières
        println!("\n--- Finances ---");
        let total_factures = self.factures.iter()
            .filter(|f| matches!(f.statut, StatutFacture::Payee))
            .fold(0.0, |acc, f| acc + f.total);
        println!("Total des factures payées: {:.2}€", total_factures);
        
        // Statistiques de la pharmacie
        println!("\n--- Pharmacie ---");
        let medicaments_alerte = self.pharmacie.medicaments.iter()
            .filter(|m| m.stock <= m.seuil_alerte)
            .count();
        println!("Médicaments en alerte stock: {}", medicaments_alerte);
        
        println!("{}", "-".repeat(40));
    }
    // Menus

    fn menu_patients(&mut self) {
        loop {
            println!("\n{}", "=== GESTION DES PATIENTS ===".blue().bold());
            println!("1. Ajouter un patient");
            println!("2. Liste des patients");
            println!("3. Retour");
            print!("\nChoix: ");
            io::stdout().flush().unwrap();

            match lire_nombre("") {
                1 => self.ajouter_patient(),
                2 => self.liste_patients(),
                3 => break,
                _ => println!("{}", "Choix invalide!".red()),
            }
        }
    }

    fn menu_personnel(&mut self) {
        loop {
            println!("\n{}", "=== GESTION DU PERSONNEL ===".blue().bold());
            println!("1. Ajouter un membre du personnel");
            println!("2. Liste du personnel");
            println!("3. Retour");
            print!("\nChoix: ");
            io::stdout().flush().unwrap();

            match lire_nombre("") {
                1 => self.ajouter_personnel(),
                2 => self.liste_personnel(),
                3 => break,
                _ => println!("{}", "Choix invalide!".red()),
            }
        }
    }

    fn menu_rendez_vous(&mut self) {
        loop {
            println!("\n{}", "=== GESTION DES RENDEZ-VOUS ===".blue().bold());
            println!("1. Nouveau rendez-vous");
            println!("2. Liste des rendez-vous");
            println!("3. Retour");
            print!("\nChoix: ");
            io::stdout().flush().unwrap();

            match lire_nombre("") {
                1 => self.ajouter_rendez_vous(),
                2 => self.liste_rendez_vous(),
                3 => break,
                _ => println!("{}", "Choix invalide!".red()),
            }
        }
    }
    
    
    fn menu_principal(&mut self) {
        loop {
            println!("\n{}", "=== GESTION HOSPITALIÈRE ===".blue().bold());
            println!("1. Gestion des Patients");
            println!("2. Gestion du Personnel");
            println!("3. Gestion des Rendez-vous");
            println!("4. Gestion des Services");
            println!("5. Gestion de la Pharmacie");
            println!("6. Gestion des Factures");
            println!("7. Administration");
            println!("8. Statistiques");
            println!("9. Quitter");
            
            match lire_nombre("\nChoix: ") {
                1 => self.menu_patients(),
                2 => self.menu_personnel(),
                3 => self.menu_rendez_vous(),
                4 => self.menu_services(),
                5 => self.menu_pharmacie(),
                6 => self.menu_factures(),
                7 => self.menu_admin(),
                8 => self.afficher_statistiques(),
                9 => break,
                _ => println!("{}", "Choix invalide!".red()),
            }
        }
    }

    fn menu_pharmacie(&mut self) {
        loop {
            println!("\n{}", "=== GESTION DE LA PHARMACIE ===".blue().bold());
            println!("1. Ajouter un médicament");
            println!("2. Vérifier les stocks");
            println!("3. Retour");
            
            match lire_nombre("\nChoix: ") {
                1 => self.ajouter_medicament(),
                2 => self.verifier_stock(),
                3 => break,
                _ => println!("{}", "Choix invalide!".red()),
            }
        }
    }

    fn menu_factures(&mut self) {
        loop {
            println!("\n{}", "=== GESTION DES FACTURES ===".blue().bold());
            println!("1. Créer une facture");
            println!("2. Liste des factures");
            println!("3. Modifier statut facture");
            println!("4. Retour");
            
            match lire_nombre("\nChoix: ") {
                1 => self.creer_facture(),
                2 => self.liste_factures(),
                3 => self.modifier_statut_facture(),
                4 => break,
                _ => println!("{}", "Choix invalide!".red()),
            }
        }
    }

    // Fonctions de modification
    fn modifier_statut_facture(&mut self) {
        self.liste_factures();
        let facture_id = lire_nombre("ID de la facture à modifier: ");
        
        if let Some(facture) = self.factures.iter_mut().find(|f| f.id == facture_id) {
            println!("\nChoisir le nouveau statut:");
            println!("1. En attente");
            println!("2. Payée");
            println!("3. Annulée");
            
            facture.statut = match lire_nombre("Choix: ") {
                1 => StatutFacture::EnAttente,
                2 => StatutFacture::Payee,
                3 => StatutFacture::Annulee,
                _ => {
                    println!("{}", "Choix invalide!".red());
                    return;
                }
            };
            
            self.save_data();
            println!("{}", "\nStatut de la facture modifié avec succès!".green());
        } else {
            println!("{}", "Facture non trouvée!".red());
        }
    }


    fn menu_admin(&mut self) {
        loop {
            println!("\n{}", "=== ADMINISTRATION ===".blue().bold());
            println!("1. Créer un utilisateur");
            println!("2. Liste des utilisateurs");
            println!("3. Sauvegarder les données");
            println!("4. Retour");
            
            match lire_nombre("\nChoix: ") {
                1 => self.creer_utilisateur(),
                2 => self.liste_utilisateurs(),
                3 => self.save_data(),
                4 => break,
                _ => println!("{}", "Choix invalide!".red()),
            }
        }
    }

    // Fonctions de listing étendues
    fn liste_services(&self) {
        println!("{}", "\n=== LISTE DES SERVICES ===".green());
        for service in &self.services {
            println!("{}", "-".repeat(40));
            println!("ID: {}", service.id);
            println!("Nom: {}", service.nom);
            if let Some(chef) = self.personnel.iter().find(|p| p.id == service.chef_service) {
                println!("Chef de service: Dr. {} {}", chef.nom, chef.prenom);
            }
            println!("Capacité: {}", service.capacite);
            println!("Personnel affecté: {}", service.personnel_affecte.len());
            println!("Équipements: {}", service.equipements.len());
        }
    }

    fn liste_factures(&self) {
        println!("{}", "\n=== LISTE DES FACTURES ===".green());
        for facture in &self.factures {
            println!("{}", "-".repeat(40));
            println!("Facture N°{}", facture.id);
            if let Some(patient) = self.patients.iter().find(|p| p.id == facture.patient_id) {
                println!("Patient: {} {}", patient.nom, patient.prenom);
            }
            println!("Date: {}", facture.date_emission);
            println!("Montant total: {:.2}€", facture.total);
            println!("Statut: {:?}", facture.statut);
        }
    }

    fn liste_utilisateurs(&self) {
        println!("{}", "\n=== LISTE DES UTILISATEURS ===".green());
        for utilisateur in &self.utilisateurs {
            println!("{}", "-".repeat(40));
            println!("ID: {}", utilisateur.id);
            println!("Nom d'utilisateur: {}", utilisateur.nom_utilisateur);
            println!("Rôle: {:?}", utilisateur.role);
            if let Some(derniere_connexion) = &utilisateur.derniere_connexion {
                println!("Dernière connexion: {}", derniere_connexion);
            }
        }
    }


    fn menu_services(&mut self) {
        loop {
            println!("\n{}", "=== GESTION DES SERVICES ===".blue().bold());
            println!("1. Ajouter un service");
            println!("2. Liste des services");
            println!("3. Retour");
            
            match lire_nombre("\nChoix: ") {
                1 => self.ajouter_service(),
                2 => self.liste_services(),
                3 => break,
                _ => println!("{}", "Choix invalide!".red()),
            }
        }
    
    }


}

// Fonctions utilitaires
fn lire_chaine(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn lire_nombre(message: &str) -> u32 {
    loop {
        let input = lire_chaine(message);
        match input.parse::<u32>() {
            Ok(n) => return n,
            Err(_) => println!("{}", "Veuillez entrer un nombre valide!".red()),
        }
    }
}

fn main() {
    let mut app = Application::charger_data();
    println!("{}", "Bienvenue dans le système de gestion hospitalière!".green().bold());
    app.menu_principal();
    println!("{}", "Au revoir!".green().bold());
}