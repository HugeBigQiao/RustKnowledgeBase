//! const 与 static：全局常量和静态变量
//!
//! let 只能在函数里定义局部变量。如果要定义"整个程序都能用的值"，
//! Rust 提供了 const（编译期常量）和 static（全局变量）。
//! 它们和 let 有本质区别，不是简单的"全局版 let"。

/// const: 编译期常量，用的时候直接内联到代码里，没有固定内存地址。
/// static: 全局变量，有固定内存地址，存活于整个程序运行期间。
/// let: 局部变量，栈上分配，离开作用域就释放。
pub fn run() {
    // ===== const：编译期常量 =====
    println!("===== const: 编译期常量 =====");

    // const 必须标注类型，值必须在编译期就能算出来。
    // 命名习惯是全大写 + 下划线。
    const MAX_SCORE: i32 = 100;
    const APP_NAME: &str = "MyApp";
    const PI: f64 = 3.1415926;

    println!("MAX_SCORE = {}", MAX_SCORE);
    println!("APP_NAME = {}", APP_NAME);
    println!("PI = {}", PI);

    // const 可以用在任意作用域——函数内、函数外、甚至 match 分支里：
    const GREETING: &str = "你好";
    println!("{}", GREETING);

    // --- const 的特点 ---
    // 1. 编译时求值：值必须在编译期确定，不能依赖运行时输入。
    //    const RUNTIME: String = format!("..."); // 编译报错！不是编译期常量
    //
    // 2. 内联：编译器把 const 的值直接替换到使用处，不分配固定内存地址。
    //    这和 C 的 #define 类似，但有关型检查。
    //
    // 3. 永远不可变：const 没有 mut 版本。想"可变全局"用 static + 同步原语。

    // const 可以基于其他 const 计算：
    const HALF_SCORE: i32 = MAX_SCORE / 2;
    println!("HALF_SCORE = {} (基于 MAX_SCORE 算出)", HALF_SCORE);

    // ===== static：全局变量 =====
    println!("\n===== static: 全局变量 =====");

    // static 有固定的内存地址，整个程序运行期间都存在。
    static LANGUAGE: &str = "Rust";
    static VERSION: u32 = 1;

    println!("语言: {}", LANGUAGE);
    println!("版本: {}", VERSION);

    // --- static 和 const 的核心区别 ---
    //   1. 内存地址：static 有固定地址，const 没有（内联了）。
    //      可以通过 &LANGUAGE 拿到 static 的地址，但拿不到 const 的。
    println!("static 的地址: {:p}", &LANGUAGE);
    // println!("{:p}", &MAX_SCORE); // const 不能取地址！

    //   2. 可变性：
    //      const 永远不可变，没有 const mut。
    //      static 可以有 static mut，但读写都需要 unsafe 块。
    //
    //   3. 初始化时机：
    //      const：编译期求值。
    //      static：类似 C 的全局变量，在程序启动时初始化（二进制里写死）。

    // ===== static mut：可变的静态变量（需 unsafe） =====
    println!("\n===== static mut: 不安全版可变全局 =====");
    static mut COUNTER: u32 = 0;

    // 读写 static mut 必须在 unsafe 块里，且 Rust 2024 要求通过裸指针操作：
    unsafe {
        let ptr = std::ptr::addr_of_mut!(COUNTER);
        *ptr += 1;
        println!("COUNTER = {} (在 unsafe 块里通过裸指针读写)", *ptr);
    }

    println!("static mut 有数据竞争风险，Rust 强迫你在 unsafe 里才用。");
    println!("实际项目建议用 Mutex<...> 或 atomic 类型替代 static mut。");

    // ===== static 的所有权与引用规则 =====
    println!("\n===== static 的所有权与引用 =====");

    // 1. static 不能持有"非 'static 生命周期"的引用。
    //    static S: &str = &String::from("x");  // 编译报错！临时值活不了多久
    //    正确：static S: &str = "字面量";        // 字面量有 'static 生命周期
    static CONFIG_PATH: &str = "/etc/app.conf";
    println!("CONFIG_PATH = {}", CONFIG_PATH);

    // 2. static 可以持有"本身有所有权"的类型，比如 String、Vec。
    //    但初始化时只能用 const 函数或字面量。复杂初始化见下面的"实际用途"。
    static APP_NAME_OWNED: &str = "MyApplication"; // &str 字面量
    println!("APP_NAME_OWNED = {}", APP_NAME_OWNED);

    // 3. static 可以被任意函数借用（&），因为它是 'static 的：
    fn print_config() {
        println!("  在另一个函数里也能访问: {}", CONFIG_PATH);
    }
    print_config();

    // 4. static 不能"移动(move)"它的值给别人。
    //    let x = CONFIG_PATH;  // 编译报错！不能移走 static 的所有权
    //    只能借(&)或用 Copy 类型（像 VERSION: u32 是 Copy，可以直接用）。

    // ===== 和 let 的区别总结 =====
    println!("\n===== let vs const vs static 对比 =====");
    println!("特性                let         const       static");
    println!("作用域              函数内      任意        模块级(全局)");
    println!("内存位置            栈上        内联        固定地址");
    println!("可变性              mut 可变    不可变       static mut(需 unsafe)");
    println!("初始化时机          运行时      编译期      程序启动时");
    println!("取地址              &x         不能        &X 可以");
    println!("持有 String/Vec     可以        不能        不能直接(需 lazy 初始化)");

    // ===== 实际用途 1：全局配置 =====
    println!("\n===== 实际用途 1: 全局配置 =====");
    println!("const 适合放\"永远不会变\"的配置:");
    println!("  const MAX_CONNECTIONS: u32 = 100;");
    println!("  const DEFAULT_TIMEOUT_SECS: u64 = 30;");
    println!("  const SUPPORTED_LANGUAGES: [&str; 3] = [\"zh\", \"en\", \"ja\"];");
    println!();
    println!("static 适合放\"有固定地址\"的值(比如被 C FFI 引用):");
    println!("  static LOG_LEVEL: &str = \"info\";");
    println!("  static PID_FILE: &str = \"/run/app.pid\";");

    // ===== 实际用途 2：全局单例 =====
    println!("\n===== 实际用途 2: 全局单例(OnceLock) =====");
    println!("如果要在 static 里放 String/Vec/HashMap 这种堆分配的类型，");
    println!("标准库提供 std::sync::OnceLock(Rust 1.70+)，经典方案有");
    println!("lazy_static 或 once_cell crate。示例:");
    println!();
    println!("  use std::sync::OnceLock;");
    println!("  static CONFIG: OnceLock<Config> = OnceLock::new();");
    println!("  fn get_config() -> &'static Config {{");
    println!("      CONFIG.get_or_init(|| Config::load())");
    println!("  }}");
    println!();
    println!("OnceLock 保证只初始化一次，之后所有线程都能安全读取。");
    println!("这是现代 Rust 推荐的全局可变状态方案，替代 static mut。");

    // ===== 选择建议 =====
    println!("\n===== 什么时候用哪个？ =====");
    println!("  const  -> \"魔法数字\"、配置常量、不需要地址的值");
    println!("  static -> 需要固定地址、C FFI 交互、OnceLock 载体");
    println!("  let    -> 其余所有情况（99% 的时候用它就够了）");
}
