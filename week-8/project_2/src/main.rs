use std::io;
use std::collections::HashMap;

// Struct to represent a job candidate
#[derive(Debug, Clone)]
struct Candidate {
    name: String,
    email: String,
    years_experience: u32,
    programming_languages: Vec<String>,
    position: String,
}

impl Candidate {
    fn new(name: &str, email: &str, years_experience: u32, languages: Vec<&str>, position: &str) -> Self {
        Candidate {
            name: name.to_string(),
            email: email.to_string(),
            years_experience,
            programming_languages: languages.into_iter().map(|s| s.to_string()).collect(),
            position: position.to_string(),
        }
    }
}

fn main() {
    println!("{}", "=".repeat(80));
    println!("{:^80}", "ERNST & YOUNG (EY) NIGERIA");
    println!("{:^80}", "DEVELOPER RECRUITMENT SYSTEM");
    println!("{}", "=".repeat(80));
    
    // Create a vector of candidates using compound data type (Vector)
    let mut candidates = vec![
        Candidate::new(
            "Adebayo Adekunle", 
            "adebayo.adekunle@email.com", 
            12, 
            vec!["Java", "Python", "SQL", "AWS"],
            "Senior Backend Developer"
        ),
        Candidate::new(
            "Chiamaka Nwosu", 
            "chiamaka.nwosu@email.com", 
            8, 
            vec!["JavaScript", "React", "Node.js", "TypeScript"],
            "Full Stack Developer"
        ),
        Candidate::new(
            "Emeka Okafor", 
            "emeka.okafor@email.com", 
            15, 
            vec!["C++", "Python", "Machine Learning", "TensorFlow"],
            "AI/ML Engineer"
        ),
        Candidate::new(
            "Fatima Bello", 
            "fatima.bello@email.com", 
            6, 
            vec!["Python", "Django", "PostgreSQL", "Docker"],
            "Backend Developer"
        ),
        Candidate::new(
            "Gabriel Okon", 
            "gabriel.okon@email.com", 
            10, 
            vec!["Go", "Kubernetes", "Microservices", "gRPC"],
            "DevOps Engineer"
        ),
        Candidate::new(
            "Zainab Yusuf", 
            "zainab.yusuf@email.com", 
            18, 
            vec!["Java", "Spring Boot", "Kafka", "Redis", "MongoDB"],
            "Principal Software Architect"
        ),
    ];

    // Display all candidates
    display_all_candidates(&candidates);
    
    // Find candidate with highest experience
    if let Some(highest_exp_candidate) = find_highest_experience(&candidates) {
        display_top_candidate(&highest_exp_candidate);
        
        // Generate experience statistics
        generate_experience_report(&candidates);
        
        // Find candidates by experience range
        find_candidates_by_experience(&candidates);
    }
    
    // Interactive mode to add new candidates
    interactive_candidate_input(&mut candidates);
}

fn display_all_candidates(candidates: &[Candidate]) {
    println!("\n📋 ALL JOB CANDIDATES");
    println!("{}", "─".repeat(120));
    println!("{:<25} {:<30} {:<8} {:<30} {:<20}", 
             "Name", "Email", "Yrs Exp", "Programming Languages", "Position");
    println!("{}", "─".repeat(120));
    
    for candidate in candidates {
        let languages = candidate.programming_languages.join(", ");
        println!("{:<25} {:<30} {:<8} {:<30} {:<20}", 
                 candidate.name, 
                 candidate.email, 
                 candidate.years_experience, 
                 languages, 
                 candidate.position);
    }
    println!("{}", "─".repeat(120));
    println!("Total Candidates: {}", candidates.len());
}

fn find_highest_experience(candidates: &[Candidate]) -> Option<&Candidate> {
    candidates.iter().max_by_key(|c| c.years_experience)
}

fn display_top_candidate(candidate: &Candidate) {
    println!("\n{}", "⭐".repeat(50));
    println!("{:^50}", "TOP CANDIDATE WITH HIGHEST EXPERIENCE");
    println!("{}", "⭐".repeat(50));
    
    println!("👤 Name: {}", candidate.name);
    println!("📧 Email: {}", candidate.email);
    println!("📅 Years of Experience: {}", candidate.years_experience);
    println!("💻 Position Applied: {}", candidate.position);
    println!("🛠️  Programming Languages: {}", candidate.programming_languages.join(", "));
    
    println!("\n🏆 RECOMMENDATION: This candidate has the highest programming experience");
    println!("   among all applicants and should be prioritized for interview.");
}

fn generate_experience_report(candidates: &[Candidate]) {
    println!("\n{}", "📊".repeat(50));
    println!("{:^50}", "EXPERIENCE DISTRIBUTION REPORT");
    println!("{}", "📊".repeat(50));
    
    let total_candidates = candidates.len();
    let total_experience: u32 = candidates.iter().map(|c| c.years_experience).sum();
    let average_experience = total_experience as f32 / total_candidates as f32;
    
    let max_experience = candidates.iter().map(|c| c.years_experience).max().unwrap_or(0);
    let min_experience = candidates.iter().map(|c| c.years_experience).min().unwrap_or(0);
    
    // Group by experience ranges using HashMap (another compound data type)
    let mut experience_ranges: HashMap<String, Vec<&Candidate>> = HashMap::new();
    
    for candidate in candidates {
        let range = match candidate.years_experience {
            0..=5 => "0-5 years",
            6..=10 => "6-10 years",
            11..=15 => "11-15 years",
            _ => "16+ years"
        }.to_string();
        
        experience_ranges.entry(range).or_insert_with(Vec::new).push(candidate);
    }
    
    println!("📈 Statistics:");
    println!("   • Total Candidates: {}", total_candidates);
    println!("   • Total Years of Experience: {} years", total_experience);
    println!("   • Average Experience: {:.1} years", average_experience);
    println!("   • Maximum Experience: {} years", max_experience);
    println!("   • Minimum Experience: {} years", min_experience);
    
    println!("\n📋 Experience Range Distribution:");
    let mut ranges: Vec<_> = experience_ranges.keys().collect();
    ranges.sort();
    
    for range in ranges {
        let candidates_in_range = &experience_ranges[range];
        println!("   • {}: {} candidates", range, candidates_in_range.len());
    }
}

fn find_candidates_by_experience(candidates: &[Candidate]) {
    println!("\n{}", "🔍".repeat(50));
    println!("{:^50}", "FIND CANDIDATES BY EXPERIENCE RANGE");
    println!("{}", "🔍".repeat(50));
    
    let senior_candidates: Vec<&Candidate> = candidates
        .iter()
        .filter(|c| c.years_experience >= 10)
        .collect();
    
    let mid_level_candidates: Vec<&Candidate> = candidates
        .iter()
        .filter(|c| c.years_experience >= 5 && c.years_experience < 10)
        .collect();
    
    println!("👨‍💼 Senior Candidates (10+ years):");
    if senior_candidates.is_empty() {
        println!("   No senior candidates found.");
    } else {
        for candidate in &senior_candidates {
            println!("   • {} ({} years) - {}", candidate.name, candidate.years_experience, candidate.position);
        }
    }
    
    println!("\n👨‍💻 Mid-Level Candidates (5-9 years):");
    if mid_level_candidates.is_empty() {
        println!("   No mid-level candidates found.");
    } else {
        for candidate in &mid_level_candidates {
            println!("   • {} ({} years) - {}", candidate.name, candidate.years_experience, candidate.position);
        }
    }
}

fn interactive_candidate_input(candidates: &mut Vec<Candidate>) {
    println!("\n{}", "➕".repeat(50));
    println!("{:^50}", "ADD NEW CANDIDATE (Interactive Mode)");
    println!("{}", "➕".repeat(50));
    
    print!("Do you want to add a new candidate? (y/n): ");
    io::Write::flush(&mut io::stdout()).expect("Flush failed");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    
    if choice.trim().to_lowercase() == "y" {
        println!("\nPlease enter candidate details:");
        
        // Get name
        print!("Full Name: ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim().to_string();
        
        // Get email
        print!("Email: ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Failed to read input");
        let email = email.trim().to_string();
        
        // Get years of experience
        print!("Years of Programming Experience: ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        let mut exp_str = String::new();
        io::stdin().read_line(&mut exp_str).expect("Failed to read input");
        let years_experience: u32 = exp_str.trim().parse().unwrap_or(0);
        
        // Get programming languages
        print!("Programming Languages (comma-separated): ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        let mut languages_str = String::new();
        io::stdin().read_line(&mut languages_str).expect("Failed to read input");
        let languages: Vec<String> = languages_str
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        
        // Get position
        print!("Position Applied For: ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        let mut position = String::new();
        io::stdin().read_line(&mut position).expect("Failed to read input");
        let position = position.trim().to_string();
        
        // Create new candidate
        let new_candidate = Candidate {
            name,
            email,
            years_experience,
            programming_languages: languages,
            position,
        };
        
        candidates.push(new_candidate.clone());
        
        println!("\n✅ New candidate added successfully!");
        println!("📋 Candidate Details:");
        println!("   Name: {}", new_candidate.name);
        println!("   Email: {}", new_candidate.email);
        println!("   Experience: {} years", new_candidate.years_experience);
        println!("   Languages: {}", new_candidate.programming_languages.join(", "));
        println!("   Position: {}", new_candidate.position);
        
        // Update top candidate after adding new one
        if let Some(highest_exp_candidate) = find_highest_experience(candidates) {
            println!("\n🔄 Updated Top Candidate: {} ({} years)", 
                     highest_exp_candidate.name, highest_exp_candidate.years_experience);
        }
    }
    
    println!("\n{}", "🎯".repeat(50));
    println!("{:^50}", "RECRUITMENT PROCESS COMPLETED");
    println!("{}", "🎯".repeat(50));
    println!("Thank you for using EY Nigeria Developer Recruitment System!");
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_highest_experience() {
        let candidates = vec![
            Candidate::new("Test1", "test1@email.com", 5, vec!["Java"], "Dev"),
            Candidate::new("Test2", "test2@email.com", 10, vec!["Python"], "Senior Dev"),
            Candidate::new("Test3", "test3@email.com", 3, vec!["JavaScript"], "Junior Dev"),
        ];
        
        let highest = find_highest_experience(&candidates).unwrap();
        assert_eq!(highest.years_experience, 10);
        assert_eq!(highest.name, "Test2");
    }

    #[test]
    fn test_empty_candidates() {
        let candidates: Vec<Candidate> = vec![];
        assert!(find_highest_experience(&candidates).is_none());
    }
}