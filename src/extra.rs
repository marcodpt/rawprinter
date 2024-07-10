pub fn escape (data: String) -> String {
    let data = data 
        .replace("á", "a")
        .replace("à", "a")
        .replace("â", "a")
        .replace("ã", "a")
        .replace("é", "e")
        .replace("è", "e")
        .replace("ê", "e")
        .replace("í", "i")
        .replace("ì", "i")
        .replace("î", "i")
        .replace("ó", "o")
        .replace("ò", "o")
        .replace("ô", "o")
        .replace("õ", "o")
        .replace("ú", "u")
        .replace("ù", "u")
        .replace("û", "u")
        .replace("Á", "A")
        .replace("À", "A")
        .replace("Â", "A")
        .replace("Ã", "A")
        .replace("É", "E")
        .replace("È", "E")
        .replace("Ê", "E")
        .replace("Í", "I")
        .replace("Ì", "I")
        .replace("Î", "I")
        .replace("Ó", "O")
        .replace("Ò", "O")
        .replace("Ô", "O")
        .replace("Õ", "O")
        .replace("Ú", "U")
        .replace("Ù", "U")
        .replace("Û", "U")
        .replace("Ç", "C")
        .replace("ç", "c");

    data.replace(|c: char| !c.is_ascii(), "")
}