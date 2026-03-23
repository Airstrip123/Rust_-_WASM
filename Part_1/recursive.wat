(module
(func $RecursiveCount (export "RecursiveCount") (param i32) (result i32)
local.get 0 
i32.const 10
i32.gt_s
if (result i32)
i32.const 0
else
local.get 0
local.get 0
i32.const 1
i32.add
call $RecursiveCount
i32.add
end))