use serde::{Deserialize, Serialize};
use texcore::{ElementList, Any, Metadata, Element, Tex, compile};
use serde_json::to_string_pretty as json_string;
use serde_json::from_str;
use std::{cell::RefCell, path::PathBuf, fs::File, io::Write};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Template{
    name: String, 
    author: String, 
    license: String, 
    description: String,
    element_list: RefCell<ElementList<Any>>, 
}


impl Template{
    pub fn new(name: &str, author: &str,license: &str,  desc: &str) -> Self{
        Self { 
            name: name.to_owned(), 
            author: author.to_owned(), 
            license: license.to_owned(), 
            description: desc.to_owned(), 
            element_list: RefCell::new(ElementList::default())
        }
    }
    pub fn from_file(path: PathBuf) -> Self{
        let content = std::fs::read_to_string(path).unwrap();
        from_str(&content).unwrap()
    }
    pub fn from_string(content: &str) -> Self{
        from_str(content).unwrap()
    }
    pub fn to_backup(&self) -> (String, String){
        let json = self.to_json();
        let name = self.name.clone();
        (name, json)
    }
    pub fn name(&self) -> String{
        self.name.clone()
    }
    pub fn push_element(&self, element: Element<Any>){
        self.element_list.borrow_mut().push(element)
    }
    pub fn push_elements(&self, elements: Vec<Element<Any>>){
        for e in elements{
            self.push_element(e)
        }
    }
    pub fn set_list(&self, list: ElementList<Any>){
        *self.element_list.borrow_mut() = list;
    }
    pub fn set_metadata(&self, metadata: Metadata){
        let list = ElementList::new(
            &metadata.author, 
            &metadata.date, 
            &metadata.title, 
            metadata.fontsize, 
            &metadata.doc_class, 
            metadata.maketitle
        );
        self.set_list(list)
    }
    pub fn to_json(&self) -> String{
        json_string(&self).unwrap()
    }
    pub fn to_html(&self) -> String {
        let mut html = Vec::new();
        html.push(format!("<h2>{}</h2>", &self.name));
        html.push(format!("<h3>By {}</h3>", &self.author));
        html.push(format!("<h4>Under {}</h4>", &self.license));
        html.push(format!("<h5>{}<h5>", &self.description));
        html.push("<code>".to_owned());
        html.push(self.to_latex_string());
        html.push("</code>".to_owned());
        html.join("\n")
    }
    pub fn write_texfiles(&self, main_path: PathBuf, incl_path: PathBuf) -> Result<(), std::io::Error>{
        let mut file = File::create(main_path)?;
        let mut incl_file = File::create(incl_path)?;
        let (main, incl) = self.element_list.borrow().clone().to_latex_split_string();
        file.write_all(main.as_bytes())?;
        incl_file.write_all(incl.as_bytes())?;
        Ok(())
    }
    pub fn write_then_compile(&self, main_path: PathBuf, incl_path: PathBuf, output_path: PathBuf) -> Result<(), std::io::Error>{
        self.write_texfiles(main_path.clone(), incl_path)?;
        compile(main_path, output_path)?;
        Ok(())
    }
}

impl Tex for Template{
    fn to_latex_string(&self) -> String {
        self.element_list.borrow().clone().to_latex_string()
    }
}