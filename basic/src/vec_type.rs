//! 向量(Vec)
//!
//! Vec 是堆上分配的可增长数组, 是 Rust 最常用的集合类型.
//! 和数组的区别: 数组定长(编译期), Vec 可以动态增减.

/// Vec 的核心特点:
/// - 所有元素必须是同一类型(`Vec<T>` 中的 T).
/// - 存在堆上, 可以在运行时增长或缩小.
/// - 有所有权: Vec 被释放时, 里面的元素也一起释放.
/// - push 将元素所有权移入 Vec; pop 将所有权移出。
/// - .get() 和 for &v 都是借用, 不取走所有权。
///
/// 关于 `<>` 尖括号(泛型):
///   `<>` 里面放的是"类型参数"——就像一个占位符, 等你填入具体类型.
///   `T` 是 Type 的缩写, 代表"任意类型". 比如:
///     `Vec<i32>`   = "元素是 i32 的 Vec"
///     `Vec<String>` = "元素是 String 的 Vec"
///   `N` 代表"任意数字(编译期常量)". 比如 `[i32; 3]` = "3 个 i32 的数组".
///   泛型后面 intermediate 会深入讲, 这里先知道 `<>` 就是"填类型参数"即可.
///
/// Vec 和数组的对比如下:
///   数组: 定长, 栈上, 类型 `[T; N]`
///   Vec:  变长, 堆上, 类型 `Vec<T>`
///
/// `vec![]` 是创建 Vec 的宏, 和数组的 `[]` 字面量类似, 但产物不同:
///   `[1, 2, 3]`  → 数组, `[i32; 3]`
///   `vec![1, 2, 3]` → Vec, `Vec<i32>`
pub fn run() {
    // ===== 创建 Vec =====
    // vec![] 宏: 最常用的创建方式。i32 是 Copy, 三个数字原样复制进堆上的 Vec。
    let v1: Vec<i32> = vec![10, 20, 30];
    println!("vec![10, 20, 30] = {:?}", v1);

    // Vec::new(): 创建空 Vec, 需要标注类型 (编译器不知道 T 是什么)。
    let mut v2: Vec<i32> = Vec::new();               // 空 Vec, 所有权归 v2
    println!("Vec::new() = {:?} (空 Vec, 长度: {})", v2, v2.len());

    // vec![值; 数量]: 创建填充了 N 个相同值的 Vec。i32 是 Copy → 数量份值被复制。
    let v3 = vec![0; 5];                             // 5 个 0
    println!("vec![0; 5] = {:?}", v3);

    // ===== 添加和删除元素 =====
    // push: 往末尾追加元素。元素所有权移入 Vec。Vec 必须有 mut 才能 push。
    v2.push(100);                                    // 100: i32 Copy — 复制一份进 Vec, 不是移走
    v2.push(200);
    v2.push(300);
    println!("push 三次后: {:?}", v2);

    // pop: 从末尾取出并移除元素。返回 Option<T> — Some(值) 所有权移出, None 表示空。
    let last = v2.pop();                             // last 拥有 300 (所有权利从 Vec 移到 last)
    println!("pop() = {:?}, 剩下: {:?}", last, v2);

    // ===== 访问元素 =====
    // 索引访问: v[i], 越界会 panic.
    println!("v1[0] = {}, v1[2] = {} (索引访问, 越界会 panic)", v1[0], v1[2]);

    // .get(i): 安全访问, 返回 Option<&T>. 越界返回 None 而不是 panic.
    println!("v1.get(0) = {:?}, v1.get(99) = {:?} (get 安全访问, 越界不 panic)", v1.get(0), v1.get(99));

    // ===== 长度和容量 =====
    // len(): 当前元素个数.
    // capacity(): 已分配的内存能容纳多少个元素(不用重新分配).
    // capacity >= len, 当 push 导致 len > capacity 时, Vec 自动扩容.
    let mut v = Vec::new();
    println!("\n初始化: len = {}, capacity = {}", v.len(), v.capacity());
    v.push(1);
    println!("push 1 后: len = {}, capacity = {}", v.len(), v.capacity());
    v.push(2);
    v.push(3);
    println!("push 3 次后: len = {}, capacity = {} (可能已自动扩容)", v.len(), v.capacity());

    // ===== Vec 的所有权 =====
    // Vec 拥有它里面的元素. Vec 被释放时, 元素一起释放.
    // 赋值 Vec 会转移所有权(move), 除非用 clone().
    let a = vec![1, 2, 3];
    let b = a;  // a 的所有权转移给了 b
    // println!("{:?}", a);  // 编译报错! a 已失效
    println!("\nb = {:?} (a 的所有权已移给 b)", b);

    // clone 复制一份:
    let c = b.clone();  // b 和 c 各自独立
    println!("b = {:?}, c = {:?} (clone 后各自独立)", b, c);

    // ===== 遍历 Vec =====
    // 借用遍历: &nums 不获取所有权, nums 之后还能用。
    let nums = vec![10, 20, 30];
    for n in &nums {                                 // n: &i32 — 每个元素借来的引用
        print!("{} ", n);
    }
    println!("<- 遍历后 nums 还能用: {:?}", nums);
    // 对比: for n in nums 会消耗 Vec, 之后 nums 不可用 (消耗性遍历)。
}
