//! const 与 static：全局常量和静态变量
//!
//! let 只能在函数里定义局部变量。如果要定义"整个程序都能用的值"，
//! Rust 提供了 const（编译期常量）和 static（全局变量）。
//! 它们和 let 有本质区别，不是简单的"全局版 let"。

/// const: 编译期常量，运行时不存在——编译器直接把值替换到使用处（内联）。
/// static: 全局变量，有固定内存地址，存活于整个程序运行期间。
/// let: 局部变量，栈上分配，离开作用域就释放。
pub fn run() {
    // ===== const：编译期常量 =====
    println!("===== const: 编译期常量 =====");
    // const 的本质：给值起个名字，编译完后名字消失，值被内联到使用处。
    // 类比：C 的 #define，但有类型检查。
    //
    // 什么叫"消失"? 不是什么东西被删了, 而是 const 从一开始就没打算存在于运行时。
    // 编译器在编译期把每一个用到 const 的地方直接替换成那个值:
    //
    //   代码:                    编译后等价于:
    //   const PORT: i32 = 8080;
    //   println!("{}", PORT);  →  println!("{}", 8080);  // PORT 三个字没了，只剩值
    //   let x = PORT + 1;      →  let x = 8080 + 1;       // 同上
    //
    // 可以理解为"全文查找替换"——不是复制一份数据放在某处然后引用它，
    // 而是像字处理器的替换功能一样, 把名字直接换成值。替换完后, 名字本身就不再需要了。
    // 二进制里没有 "PORT 这个变量", 只有散落在各处的数字 8080。
    //
    // 所以 const 运行时不存在 = 无法取地址(&)、无法修改、不占任何内存。

    const MAX_SCORE: i32 = 100;
    const APP_NAME: &str = "MyApp";
    const PI: f64 = 3.1415926;

    // 编译后这三行等价于：
    //   println!("MAX_SCORE = {}", 100i32);
    //   println!("APP_NAME = {}", "MyApp");
    //   println!("PI = {}", 3.1415926f64);
    println!("MAX_SCORE = {}", MAX_SCORE);
    println!("APP_NAME = {}", APP_NAME);
    println!("PI = {}", PI);

    // const 可以用在任意作用域——函数内、函数外、甚至 match 分支里。
    // static 做不到，因为 static 只能在模块级定义。
    const GREETING: &str = "你好";
    println!("{}", GREETING);

    // --- const 的关键特性 ---
    //
    // 1. 运行时不存在：没有内存地址，没有栈/堆分配，纯粹是编译期替换。
    //    println!("{:p}", &MAX_SCORE);  // 编译报错！const 没有地址可指
    //
    // 2. 永远不可变：没有 const mut 这种东西。想"可变全局"用 static + OnceLock。
    //
    // 3. 编译时求值：值必须编译期确定。
    //    const RUNTIME: String = format!("...");  // 编译报错！
    //
    // 4. 可以基于其他 const 计算（编译器在编译期算完）：
    const HALF_SCORE: i32 = MAX_SCORE / 2; // 编译期算出 100 / 2 = 50
    println!("HALF_SCORE = {} (编译期算出 MAX_SCORE / 2)", HALF_SCORE);

    // -------- 到这里，你可能觉得 const 和 static 差不多？看下面 --------

    // ===== static：有地址的全局变量 =====
    println!("\n===== static: 有地址的全局变量 =====");
    // static 在程序的数据段里有真实的存储空间，有固定的内存地址。
    // 类比：C 的全局变量。编译器不能把它的值"替换掉"，因为可能有外部代码
    // 引用它的地址。每次访问 static 都是一次真实的"从内存里读"。

    static LANGUAGE: &str = "Rust";
    static VERSION: u32 = 1;

    println!("语言: {}", LANGUAGE);
    println!("版本: {}", VERSION);

    // ── 关键区别：static 可以取地址 ──
    println!("static 地址: {:p}", &LANGUAGE); // 真实地址，指向数据段
    // &MAX_SCORE 会编译报错，因为 const 运行时不存在，指不到任何地方

    // ── 关键区别：static 不能被内联 ──
    // 下面代码每次访问 VERSION 都要从内存读取。
    // 如果是 const，每次访问都是直接用一个立即数，更快。

    // ── 特殊用法：static 可以跨函数直接访问（不用传参） ──
    fn print_lang() {
        println!("  另一个函数直接访问: {}", LANGUAGE); // 不需要传参！
    }
    print_lang();

    // ===== static mut：可变的静态变量（不推荐） =====
    println!("\n===== static mut: 不推荐 =====");
    static mut COUNTER: u32 = 0;

    // 读写 static mut 必须在 unsafe 块，且 Rust 2024 要求通过裸指针。
    // 这是特意设的障碍：提醒你"这很危险，别随便用"。
    unsafe {
        let ptr = std::ptr::addr_of_mut!(COUNTER);
        *ptr += 1;
        println!("COUNTER = {} (裸指针读写)", *ptr);
    }
    println!("数据竞争风险极高，实际项目用 Mutex 或 atomic 替代 static mut。");

    // ===== static 的所有权与引用规则 =====
    println!("\n===== static 的所有权与引用 =====");

    // static 里的引用必须自身也是 'static 的（能活到程序结束）。
    static CONFIG_PATH: &str = "/etc/app.conf"; // OK：字面量是 'static
    // static S: &str = &String::from("x");       // 编译报错！临时 String 活不了那么久
    println!("CONFIG_PATH = {} (引用必须是 'static)", CONFIG_PATH);

    // static 不能被 move！不能把 static 里的值"搬走"。
    // let x = CONFIG_PATH;  // 编译报错！不能搬
    // 只能借用 (&) 或直接使用 Copy 类型（u32 是 Copy）。

    // ===== 一句话区分 =====
    println!("\n===== 一句话区分：const 是\"值的别名\"，static 是\"内存里的变量\" =====");

    // ===== 对照表 =====
    println!("\n===== 对照表 =====");
    println!("                const               static");
    println!("原理            查找替换            存数据段");
    println!("运行时存在？     否（编译完消失）    是（内存里）");
    println!("有地址？         否                  是（可以 & 取地址）");
    println!("性能            更快（立即数）       需要内存读取");
    println!("定义位置        任意作用域           只能模块级");
    println!("可变           否                   需要 unsafe");
    println!("持有 String？    否（编译期常量）     否（需 OnceLock）");

    // ===== 实际用途：什么时候用哪个？ =====
    println!("\n===== 实际用途 =====");

    println!("const 的场景（绝大多数情况）：");
    println!("  const MAX_CONNS: u32 = 100;          // 魔法数字");
    println!("  const APP_NAME: &str = \"MyApp\";     // 程序版本名");
    println!("  const TIMEOUT: u64 = 30;             // 超时时间");

    println!("\nstatic 的场景（需要\"地址\"的时候）：");
    println!("  // 1. 和 C 代码交互，C 需要知道变量的地址");
    println!("  static HOST: &str = \"127.0.0.1\";");
    println!();
    println!("  // 2. 惰性初始化的全局单例（实际项目主流方案）");
    println!("  use std::sync::OnceLock;");
    println!("  static CONFIG: OnceLock<Config> = OnceLock::new();");
    println!("  fn get_config() -> &'static Config {{");
    println!("      CONFIG.get_or_init(|| Config::load_from_file())");
    println!("  }}");

    println!("\nlet 的场景（其余一切）：");
    println!("  函数内的局部变量、循环计数器、临时中间结果……");

    println!("\n一句话：优先 let，全局不变用 const，需要地址才用 static。");
}
