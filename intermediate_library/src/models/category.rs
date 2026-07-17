/// 图书分类枚举。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Fiction,
    Science,
    History,
    Technology,
    Philosophy,
    /// 其他分类, 携带自定义名称.
    Other(String),
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Fiction => write!(f, "小说"),
            Category::Science => write!(f, "科学"),
            Category::History => write!(f, "历史"),
            Category::Technology => write!(f, "技术"),
            Category::Philosophy => write!(f, "哲学"),
            Category::Other(s) => write!(f, "其他({})", s),
        }
    }
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "小说" | "fiction" => Category::Fiction,
            "科学" | "science" => Category::Science,
            "历史" | "history" => Category::History,
            "技术" | "technology" => Category::Technology,
            "哲学" | "philosophy" => Category::Philosophy,
            other => Category::Other(other.to_string()),
        }
    }
}
