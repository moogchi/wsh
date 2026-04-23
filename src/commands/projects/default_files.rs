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
