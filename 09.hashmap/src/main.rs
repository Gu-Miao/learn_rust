use std::collections::HashMap;

fn main() {
  // HashMap<K, V>
  // 以键值对的形式存储对象，它是同构的，即所有键是同一种类型，所有的值也是同一种类型
  let mut hm1 = HashMap::new(); // 类型推断同上
  hm1.insert(String::from("red"), 0);

  // 或者可以通过在元素类型为元组的 vector 上使用 collect 方法创建 HashMap
  // 要求元组有两个值，一个为键，一个为值
  // collect 方法可以创建很多类型，因此需要显式指定类型为 HashMap
  let teams = vec![String::from("blue"), String::from("red")];
  let scores = vec![125, 117];
  let arr = teams.iter().zip(scores.iter()); // 创建元组的 vector
  let _hm2: HashMap<_, _> = arr.collect();

  // 所有权
  // 对于实现了 Copy trait 的类型，值会被复制到 HashMap 中
  // 对于拥有所有权的类型，值会被移动，所有权会转移给 HashMap
  let hm_key = String::from("red");
  let hm_val = String::from("#ff0000");
  let mut hm3 = HashMap::new();

  // hm3.insert(hm_key, hm_val); // 所有权转移
  // println!("{}, {}", hm_key, hm_val); // 报错！

  hm3.insert(&hm_key, &hm_val); // 传递引用
  println!("{}, {}", hm_key, hm_val); // 所有权未转移，正常使用

  // 如果将值的引用插入 HashMap，在 HashMap 有效期间，被引用的值也必须有效

  // 访问 HashMap 中的值
  // 可以用 get 方法，它会返回一个 Option 枚举
  let hm3_val = hm3.get(&hm_key);
  match hm3_val {
    Some(val) => println!("hm3_val is {}", val),
    None => println!("Can not get hm3_val"),
  }

  // 遍历 HashMap
  // 这里使用了一个类似解构的模式匹配
  for (k, v) in &hm3 {
    println!("key is {}, val is {}", k, v);
  }

  // 向 HashMap 中插入一对键值，如果键已存在，但值不相同，那么新值会替换旧值
  let mut hm4 = HashMap::new();
  hm4.insert("red".to_string(), "#f00".to_string());
  hm4.insert("red".to_string(), "#ff0000".to_string()); // 替换 red 的值

  println!("{:?}", hm4); // red 为 #ff0000

  // 只有在键不存在时，才向 HashMap 中插入
  // 可以使用 entry 方法，返回一个 Entry 枚举
  // 再使用 or_insert 方法，如果键存在，它会返回值的可变引用；如果键不存在，它会将新的键值对
  // 插入再返回新值的可变引用
  hm4
    .entry("red".to_string())
    .or_insert("rgb(255, 0, 0)".to_string()); // red 存在，不会插入
  hm4
    .entry("blue".to_string())
    .or_insert("rgb(0, 0, 255)".to_string()); // blue 不存在，插入

  println!("after entry, hm4 is {:?}", hm4);

  // 更新现有值
  let text = "hello world man girl man hello world hello";
  let mut hm5 = HashMap::new();

  for word in text.split_whitespace() {
    // 如果 word 不存在，就插入 { [word]: 0}
    let count = hm5.entry(word).or_insert(0);
    *count += 1; // 累加
  }
  // split_whitespace 方法可以将字符串切片按空格分割并返回一个遍历器
  // 如果 word 不存在，向 HashMap 中添加，并将值初始化为 0
  // 再使用解引用将值 +1

  println!("hm5 is {:?}", hm5);
}
