pub fn search<'a>(query: &str, contents: &'a str ) -> Vec<&'a str> {

    let mut res = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productiove.
Pick there.";
        assert_eq!(vec!["safe, fast, productiove."], search(query, contents));
    }

}