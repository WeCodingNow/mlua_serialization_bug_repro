local fun = require('fun')

local repro = require('repro')


local function uses_luafun(arg)
    local args = arg.simple_args

    local ok, ret = pcall(function()
        return fun.iter(args):totable()
    end)

    if not ok then
        print(("Error: %s"):format(ret))

        return
    end

    print(ret)
end

rawset(_G, "uses_luafun", uses_luafun)

repro.call_function_with_container("uses_luafun")
