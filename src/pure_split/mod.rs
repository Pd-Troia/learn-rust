pub fn pure_split(str:&String,ch:char)->Vec<&str>{
    let mut i = 0;
    let mut j = 0;
    let mut splited : Vec<&str> = Vec::new();
    for (idx,byte) in str.bytes().enumerate(){
        if idx == str.len()-1{
            splited.push(&str[j..]);
        }
        if byte == ch as u8 {
            splited.push(&str[j..i]);
            j = i + 1 
        }
        i = i + 1;
    }
    splited
}