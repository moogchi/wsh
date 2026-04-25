pub const DEFAULT_CPP_FILE: &str = r#"#include <iostream>

using namespace std;
int main(){
    cout << "Hello, Wustite!" << endl;
    return 0;
}
"#;

pub const DEFAULT_C_FILE: &str = r#"#include <stdio.h>

int main(){
    printf("Hello, Wustite!\n");
    return 0;
}
"#;

pub const DEFAULT_PYTHON_FILE: &str = r#"
if __name__ == "__main__":
    print("Hello, Wustite!");
"#;

pub const DEFAULT_RUST_FILE: &str = r#"fn main() {
    println!("Hello, Wustite!");
}
"#;

pub const DEFAULT_JS_FILE: &str = r#"console.log("Hello, Wustite!");
"#;

pub const DEFAULT_JAVA_FILE: &str = r#"public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, Wustite!");
    }
}
"#;
