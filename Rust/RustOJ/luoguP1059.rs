//https://www.luogu.com.cn/problem/P1059

use std::io::stdin;

fn main(){
    let mut a :[i32; 1005] = [0; 1005];
    let mut ans = 0;
    let mut input = String::new();
    stdin().read_line(&mut input);
    // println!("{}", input);
    let n = input.trim().parse::<i32>().unwrap();
    //println!("n: {}", n);
    input.clear();
    stdin().read_line(&mut input);
    let mut iter = input.trim().split_whitespace();
    for it in iter{
        //println!("{}", it.trim().parse::<i32>().unwrap());
        let x = it.trim().parse::<i32>().unwrap() as usize;
        if(a[x] == 0){
            ans += 1;
        }
        a[x] = 1;
    }
    println!("{}", ans);
    for i in 0..1005{
        if(a[i] == 1){
            print!("{} ", i);
        }
    }
}


//C实现：
// #include <bits/stdc++.h>
// using namespace std;

// int arr[1005];

// int main(void){
// 	int n;
// 	scanf("%d", &n);
// 	for(int i = 0; i < n; i++){
// 		int temp;
// 		scanf("%d", &temp);
// 		arr[temp] = 1;
// 	}
// 	int ans = 0;
// 	for(int i = 0; i < 1001; i++){
// 		if(arr[i] == 1)
// 			ans++;
// 	}
// 	printf("%d\n", ans);
	
// 	for(int i = 0; i < 1001; i++){
// 		if(arr[i] == 1)
// 			printf("%d ", i);
// 	}
// }