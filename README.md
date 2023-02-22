


cargo run --bin client_reqwest

## 测试记录

## 第一次试验(100ms 请求一次,请求100次，进行json解析)
ave cost = 748.057µs
ave cost = 776.064µs
ave cost = 826.657µs
ave cost = 825.99µs
ave cost = 798.619µs

794.6us

## 第二次试验(连续请求100次，进行json解析)
ave cost = 423.065µs
ave cost = 423.485µs
ave cost = 490.671µs
ave cost = 512.955µs
ave cost = 430.626µs

455.6us

## 第三次试验(连续请求100次，不进行json解析)
ave cost = 439.275µs
ave cost = 408.595µs
ave cost = 369.347µs
ave cost = 390.571µs
ave cost = 441.685µs

409us