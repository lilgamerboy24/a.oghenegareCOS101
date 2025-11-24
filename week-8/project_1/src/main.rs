use std::io;

// Struct to represent a public servant
#[derive(Debug)]
struct PublicServant {
    name: String,
    profession: Profession,
    years_experience: u32,
}

// Enum for different professions
#[derive(Debug)]
enum Profession {
    OfficeAdministrator,
    Academic,
    Lawyer,
    Teacher,
}

// Struct to hold APS level information
#[derive(Debug)]
struct APSLevel {
    level: String,
    position: String,
    min_experience: u32,
    max_experience: u32,
}

impl PublicServant {
    fn new(name: String, profession: Profession, years_experience: u32) -> Self {
        PublicServant {
            name,
            profession,
            years_experience,
        }
    }

    fn get_aps_level(&self) -> Option<APSLevel> {
        match self.profession {
            Profession::OfficeAdministrator => get_office_admin_level(self.years_experience),
            Profession::Academic => get_academic_level(self.years_experience),
            Profession::Lawyer => get_lawyer_level(self.years_experience),
            Profession::Teacher => get_teacher_level(self.years_experience),
        }
    }
}

// Office Administrator levels
fn get_office_admin_level(experience: u32) -> Option<APSLevel> {
    let levels = vec![
        APSLevel { level: "APS 1-2".to_string(), position: "Intern".to_string(), min_experience: 0, max_experience: 2 },
        APSLevel { level: "APS 3-5".to_string(), position: "Administrator".to_string(), min_experience: 3, max_experience: 5 },
        APSLevel { level: "APS 5-8".to_string(), position: "Senior Administrator".to_string(), min_experience: 5, max_experience: 8 },
        APSLevel { level: "EL1 8-10".to_string(), position: "Office Manager".to_string(), min_experience: 8, max_experience: 10 },
        APSLevel { level: "EL2 10-13".to_string(), position: "Director".to_string(), min_experience: 10, max_experience: 13 },
        APSLevel { level: "SES".to_string(), position: "CEO".to_string(), min_experience: 13, max_experience: 50 },
    ];
    
    levels.into_iter().find(|level| experience >= level.min_experience && experience <= level.max_experience)
}

// Academic levels
fn get_academic_level(experience: u32) -> Option<APSLevel> {
    let levels = vec![
        APSLevel { level: "APS 3-5".to_string(), position: "Research Assistant".to_string(), min_experience: 3, max_experience: 5 },
        APSLevel { level: "APS 5-8".to_string(), position: "PhD Candidate".to_string(), min_experience: 5, max_experience: 8 },
        APSLevel { level: "EL1 8-10".to_string(), position: "Post-Doc Researcher".to_string(), min_experience: 8, max_experience: 10 },
        APSLevel { level: "EL2 10-13".to_string(), position: "Senior Lecturer".to_string(), min_experience: 10, max_experience: 13 },
        APSLevel { level: "SES".to_string(), position: "Dean".to_string(), min_experience: 13, max_experience: 50 },
    ];
    
    levels.into_iter().find(|level| experience >= level.min_experience && experience <= level.max_experience)
}

// Lawyer levels
fn get_lawyer_level(experience: u32) -> Option<APSLevel> {
    let levels = vec![
        APSLevel { level: "APS 1-2".to_string(), position: "Paralegal".to_string(), min_experience: 0, max_experience: 2 },
        APSLevel { level: "APS 3-5".to_string(), position: "Junior Associate".to_string(), min_experience: 3, max_experience: 5 },
        APSLevel { level: "APS 5-8".to_string(), position: "Associate".to_string(), min_experience: 5, max_experience: 8 },
        APSLevel { level: "EL1 8-10".to_string(), position: "Senior Associate 1-2".to_string(), min_experience: 8, max_experience: 10 },
        APSLevel { level: "EL2 10-13".to_string(), position: "Senior Associate 3-4".to_string(), min_experience: 10, max_experience: 13 },
        APSLevel { level: "SES".to_string(), position: "Partner".to_string(), min_experience: 13, max_experience: 50 },
    ];
    
    levels.into_iter().find(|level| experience >= level.min_experience && experience <= level.max_experience)
}

// Teacher levels
fn get_teacher_level(experience: u32) -> Option<APSLevel> {
    let levels = vec![
        APSLevel { level: "APS 1-2".to_string(), position: "Placement".to_string(), min_experience: 0, max_experience: 2 },
        APSLevel { level: "APS 3-5".to_string(), position: "Classroom Teacher".to_string(), min_experience: 3, max_experience: 5 },
        APSLevel { level: "APS 5-8".to_string(), position: "Senior Teacher".to_string(), min_experience: 5, max_experience: 8 },
        APSLevel { level: "EL1 8-10".to_string(), position: "Leading Teacher".to_string(), min_experience: 8, max_experience: 10 },
        APSLevel { level: "EL2 10-13".to_string(), position: "Deputy Principal".to_string(), min_experience: 10, max_experience: 13 },
        APSLevel { level: "SES".to_string(), position: "Principal".to_string(), min_experience: 13, max_experience: 50 },
    ];
    
    levels.into_iter().find(|level| experience >= level.min_experience && experience <= level.max_experience)
}

fn display_profession_menu() {
    println!("\n📋 Select Profession:");
    println!("1. Office Administrator");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");
}

fn get_profession_choice() -> Option<Profession> {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    
    match choice.trim() {
        "1" => Some(Profession::OfficeAdministrator),
        "2" => Some(Profession::Academic),
        "3" => Some(Profession::Lawyer),
        "4" => Some(Profession::Teacher),
        _ => None,
    }
}

fn main() {
    println!("{}", "=".repeat(70));
    println!("{:^70}", "FEDERAL GOVERNMENT OF NIGERIA");
    println!("{:^70}", "PUBLIC SERVICE APS LEVEL CHECKER");
    println!("{}", "=".repeat(70));
    
    loop {
        println!("\n👤 Enter Staff Details:");
        
        // Get staff name
        print!("Staff Name: ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim().to_string();
        
        if name.is_empty() {
            println!("❌ Name cannot be empty!");
            continue;
        }
        
        // Get profession
        display_profession_menu();
        print!("Select profession (1-4): ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        
        let profession = match get_profession_choice() {
            Some(prof) => prof,
            None => {
                println!("❌ Invalid profession selection!");
                continue;
            }
        };
        
        // Get years of experience
        print!("Years of Work Experience: ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        let mut experience_str = String::new();
        io::stdin().read_line(&mut experience_str).expect("Failed to read input");
        
        let years_experience: u32 = match experience_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number for experience!");
                continue;
            }
        };
        
        // Create public servant and get APS level
        let staff = PublicServant::new(name, profession, years_experience);
        
        println!("\n{}", "─".repeat(70));
        println!("📊 STAFF APS LEVEL VALIDATION RESULT");
        println!("{}", "─".repeat(70));
        
        match staff.get_aps_level() {
            Some(aps_level) => {
                println!("👤 Staff Name: {}", staff.name);
                println!("💼 Profession: {:?}", staff.profession);
                println!("📅 Years of Experience: {}", staff.years_experience);
                println!("🏆 APS Level: {}", aps_level.level);
                println!("📋 Position: {}", aps_level.position);
                println!("✅ Validation: PASSED");
            }
            None => {
                println!("❌ No APS level found for the given experience.");
                println!("💡 Please check if the experience value is within valid range.");
            }
        }
        
        // Ask if user wants to check another staff
        println!("\n{}", "─".repeat(70));
        print!("Check another staff? (y/n): ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        
        let mut continue_choice = String::new();
        io::stdin().read_line(&mut continue_choice).expect("Failed to read input");
        
        if continue_choice.trim().to_lowercase() != "y" {
            println!("\n👋 Thank you for using the Nigerian Public Service APS Level Checker!");
            break;
        }
    }
}

// Unit tests for the APS level checker
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lawyer_associate_aps5_8() {
        let lawyer = PublicServant::new(
            "John Doe".to_string(),
            Profession::Lawyer,
            6
        );
        
        let aps_level = lawyer.get_aps_level().unwrap();
        assert_eq!(aps_level.level, "APS 5-8");
        assert_eq!(aps_level.position, "Associate");
    }

    #[test]
    fn test_teacher_placement_aps1_2() {
        let teacher = PublicServant::new(
            "Jane Smith".to_string(),
            Profession::Teacher,
            1
        );
        
        let aps_level = teacher.get_aps_level().unwrap();
        assert_eq!(aps_level.level, "APS 1-2");
        assert_eq!(aps_level.position, "Placement");
    }
}