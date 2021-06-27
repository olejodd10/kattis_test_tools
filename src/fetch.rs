use std::path::{Path, PathBuf};

pub fn fetch_test_cases(problem_name: &str, test_cases_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use curl::easy::Easy;
    
    let test_cases_dir = PathBuf::from(test_cases_dir); //Må gjøre sånn her så en &Path ikke borrowes inn i closuren under

    let mut easy = Easy::new();
    easy.url(
        &format!("https://open.kattis.com/problems/{}/file/statement/samples.zip", problem_name)
    )?;
    easy.write_function(move |data| { 
        zip_extract::extract(std::io::Cursor::new(data), &test_cases_dir, true).expect("Unzipping error");
        Ok(data.len())
    })?;
    easy.perform()?;

    Ok(())
}