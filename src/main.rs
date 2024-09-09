mod smal;

fn main() {
    smal::add("data.smal", "personalidade", "--ignorewhitespaces --ignorestarts:#","\
        \nPersonalidade muito boa\
        \n# isso será um comentário devido ao \"--ignorestarts#\"\
        \nn sei oq, oq lá\
        \n# o espaço abaixo será ignorado pois \"--ignorewhitespaces\"\
        \n\
        \nblá blá blá\
        ");
    println!("{}", smal::read("data.smal", "personalidade", "raw"));
    smal::remove("data.smal", "personalidade");
}
