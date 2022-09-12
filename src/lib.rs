use serde::{Deserialize, Serialize};
use texcore::{ElementList, Any, Metadata, Element, Tex};
use serde_json::to_string_pretty as json_string;
use std::cell::RefCell;

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
    pub fn push_element(&self, element: Element<Any>){
        self.element_list.borrow_mut().push(element)
    }
    pub fn set_list(&self, list: ElementList<Any>){
        *self.element_list.borrow_mut() = list;
    }
    pub fn set_metadata(&self, metadata: Metadata){
        let list = ElementList::new(
            metadata.author.clone(), 
            metadata.date.clone(), 
            metadata.title.clone(), 
            metadata.fontsize, 
            metadata.doc_class.clone(), 
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
}

impl Tex for Template{
    fn to_latex_string(&self) -> String {
        self.element_list.borrow().clone().to_latex_string()
    }
}