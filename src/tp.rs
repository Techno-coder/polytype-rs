use itertools::Itertools;

pub trait Type {
    fn is_polymorphic(&self) -> bool;
    fn as_arrow(&self) -> Option<&Arrow>;
    /// Supplying is_return helps arrows look cleaner.
    fn show(&self, is_return: bool) -> String;
}

pub struct Arrow {
    pub arg: Box<Type>,
    pub ret: Box<Type>,
}
impl Type for Arrow {
    fn is_polymorphic(&self) -> bool {
        self.arg.is_polymorphic() || self.ret.is_polymorphic()
    }
    fn as_arrow(&self) -> Option<&Arrow> {
        Some(&self)
    }
    fn show(&self, is_return: bool) -> String {
        if is_return {
            format!("{} → {}", self.arg.show(false), self.ret.show(true))
        } else {
            format!("({} → {})", self.arg.show(false), self.ret.show(true))
        }
    }
}
impl Arrow {
    pub fn arg_types(&self) -> Vec<&Box<Type>> {
        if let Some(arrow) = self.ret.as_arrow() {
            let mut tps = arrow.arg_types();
            tps.insert(0, &self.arg);
            tps
        } else {
            vec![&self.arg]
        }
    }
    pub fn return_type(&self) -> &Box<Type> {
        if let Some(arrow) = self.ret.as_arrow() {
            arrow.return_type()
        } else {
            &self.ret
        }
    }
}

pub struct Constructed {
    pub name: String,
    pub args: Vec<Box<Type>>,
}
impl Type for Constructed {
    fn is_polymorphic(&self) -> bool {
        self.args.iter().any(|t| t.is_polymorphic())
    }
    fn as_arrow(&self) -> Option<&Arrow> {
        None
    }
    fn show(&self, _is_return: bool) -> String {
        if self.args.is_empty() {
            self.name.clone()
        } else {
            format!(
                "{}({})",
                &self.name,
                self.args.iter().map(|t| t.show(true)).join(",")
            )
        }
    }
}

pub struct Variable {
    pub number: u32,
}
impl Type for Variable {
    fn is_polymorphic(&self) -> bool {
        true
    }
    fn as_arrow(&self) -> Option<&Arrow> {
        None
    }
    fn show(&self, _is_return: bool) -> String {
        format!("t{}", self.number)
    }
}