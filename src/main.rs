use std::io;

 const VOWEL_SUFFIX: &str = "-hay";
 const CONSONANT_SUFFIX: &str = "ay";


fn main() {

    loop {
        println!("Let us talk pig_latin? [y/n]");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("failed to read line");

        if answer.trim().to_lowercase() == "y" {
            println!("Please insert a sentence");
            let mut s = String::new();
            io::stdin()
                .read_line(&mut s)
                .expect("failed to read line");

            if s.trim().len()==0 {
                continue;
            } else {
                let mut s_pig_latin = String::new();
                let vowels = ['a', 'e', 'i', 'o', 'u'];
                for w in s.split(" "){
                    let starts_with_vowel = vowels.iter().any(|v| w.starts_with(*v));
                    
                    
                    if starts_with_vowel {
                        s_pig_latin.push_str(w.trim());
                        s_pig_latin.push_str(VOWEL_SUFFIX);
                    } else {
                        let mut counter = 0;
                        let mut first_char = String::new();
                        for c in w.trim().chars() {
                            if counter > 0 {
                                s_pig_latin.push(c);
                            } else {
                                first_char.push(c);
                            }
                            counter+=1;
                        }
                        s_pig_latin.push('-');
                        s_pig_latin.push_str(&first_char);
                        s_pig_latin.push_str(CONSONANT_SUFFIX);
                    }
                    
                    s_pig_latin.push(' ');
                }
                println!("{}", s_pig_latin);
            }
        }
        else {
            println!("ok we won't talk pig latin");
            break;
        }
    }

}
