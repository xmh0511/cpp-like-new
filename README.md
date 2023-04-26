#### Description
1. Support to create a single object through `new`, as well as, destroy the object through `delete`
2. Support to create an array object through `new`, as well as, destroy the array through `delete []`
3. Support multiple dimension array with the syntax of C++
4. For an array object, if the initializer expressions are less than the size of the created array, the remaining elements are default initialized
5. Notice that all functions/macros in this library are `unsafe`(i.e. use them in unsafe context/block)

#### How to use this library

> Create a single object like the way in C++
````rust
let ptr = new!{ i32 }; // with default initialization
let ptr_init = new!{ i32 {10} }; // with 10 as its initializer

// destroy the created objects
delete!{ ptr };  
delete!{ ptr_init }; 
````

> Create an array object 
````rust
let size = 2;
let arr = new!{ i32[size] };
let arr_init = new!{ i32[size]{0,1}};

let mul_dim_arr = new!{  i32[size][3][4] };
let two_dim_arr = new!{ i32[size][3] {[0,0,0], [1,2,3]} };

// destroy the created objects
delete!{ [] arr};
delete!{ [] arr_init};
delete!{ [] mul_dim_arr};
delete!{ [] two_dim_arr};
````