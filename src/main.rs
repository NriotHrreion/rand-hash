use cli_clipboard::{ClipboardContext, ClipboardProvider};
use openssl::{hash::{self, MessageDigest}, rand};

fn main() {
    let mut str_list: Vec<String> = vec![];

    generate_rand_hash(MessageDigest::md5(), "MD5", &mut str_list);
    generate_rand_hash(MessageDigest::sha1(), "SHA1", &mut str_list);
    generate_rand_hash(MessageDigest::sha224(), "SHA224", &mut str_list);
    generate_rand_hash(MessageDigest::sha256(), "SHA256", &mut str_list);

    let mut ctx = ClipboardContext::new().unwrap();
    match ctx.set_contents(str_list.join("\n")) {
        Ok(()) => println!("已复制到剪贴板"),
        Err(_) => println!("无法复制到剪贴板")
    }

    let term = console::Term::stdout();
    if term.is_term() {
        term.write_line("按任意键继续...").unwrap();
        term.read_key().unwrap();
    }
}

fn generate_rand_hash(t: MessageDigest, display_name: &str, str_list: &mut Vec<String>) {
    let mut rand_buf = [0; 256];
    rand::rand_bytes(&mut rand_buf).unwrap();

    match hash::hash(t, &rand_buf) {
        Ok(result) => {
            let hash_str = format!("{}: {}", display_name, hex::encode(result.to_vec()));
            println!("{}", hash_str);
            str_list.push(hash_str)
        },
        Err(err) => println!("{}", err)
    }
}
