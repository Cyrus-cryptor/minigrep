pub fn search<'a>(query: &str, contents: &'a str ) -> Vec<&'a str> {

    contents.lines()
            .filter(|e| e.contains(query))
            .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = &query.to_lowercase();
    contents.lines()
            .filter(|line| line.to_lowercase().contains(query))
            .collect()
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(query) {
    //         res.push(line);
    //     }
    // }
    // res
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

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}