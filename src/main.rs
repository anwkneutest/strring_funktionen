fn main() {

    let  mut text: String =  String::from("\nDas ist ein Beispieltext");
    println!("{}", text);

    text.push_str(" erweitert um einen weiteren Halbsatz");
    println!("{}", text);

    text.push('!');
    println!("{}", text);

    text += " Und noch ein Satz.";
    println!("{}", text);

    let zusatz_text: String = String::from(" mit weiterem Text.");

    //text += &zusatztext;
    //println!("{}", text);
    
    text.push_str(String::as_str(&zusatz_text));
    println!("{}", text);

    println!("\n --- Anzahl der zeichen in 'text': {}", text.len());

    let mut neuer_text: String = String::from("Das ist ein neuer Text!");
    println!("\n{}", neuer_text);
    println!("\n{}", neuer_text.to_uppercase());
    println!("{}", neuer_text.to_lowercase());

    neuer_text.pop();
    println!("neuer_text.pop():{}", neuer_text);
    neuer_text.remove(6);
    println!("neuer_text.remove(idx: 6): {}", neuer_text);
    neuer_text.insert(6, 't');
    println!("neuer_text.insert(idx: 6, ch: 't'): {}", neuer_text);
    neuer_text.insert_str(11, " absolut");
    println!("neuer_text.insert_str(idx: 11, \" absolut\"): {}", neuer_text);
    neuer_text = neuer_text.replace("absolut", "wirklich");
    println!("neuer_text.replace(\"absolut\", \"wirklich\"): {}", neuer_text);
    neuer_text = "Tschüss".to_string();
    println!("neuer_text = \"Tschüss\".to_string(): {}", neuer_text);
    

}
