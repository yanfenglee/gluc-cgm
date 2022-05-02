import requests
from jsonpath import jsonpath
import re
from datetime import datetime

# retrieve data from json path
def jp(data, path, default=None):
    v = jsonpath(data, path)
    if v and type(v) is list:
        return v[0]
    return default


# replace like $.aaa.bbb in str by ctx
def var_replace(src: str, ctx: dict):
    regex = r'(\$(?:\.\w+)*)'
    if re.fullmatch(regex, src):
        return jp(ctx, src)

    keys = re.findall(regex, src)
    for k in keys:
        val = jp(ctx,k,'')
        src = src.replace(k, str(val))

    return src


# recursive resolve jsonpath value with context
def resolve(val, ctx):
    if type(val) is str:
        return var_replace(val, ctx)
    elif type(val) in [int,float,bool]:
        return val
    elif type(val) is dict:
        new_dict = {}
        for k in val:
            new_dict[k] = resolve(val[k], ctx)
        return new_dict
    elif type(val) is list:
        new_list = []
        for v in val:
            new_list.append(resolve(v, ctx))
        return new_list
    else:
        return val


# if result as expect
def match(a, b):
    if type(a) is dict and type(b) is dict:
        keys = set(a.keys()) & set(b.keys())
        return [a[k]==b[k] for k in keys].count(False) == 0
    else:
        return a == b
    

# test single, add response to context
def apply_single(ts, ctx):
    resp = {}
    try:
        method = resolve(ts['method'], ctx)
        url = resolve(ts['url'], ctx)
        params = resolve(ts['params'], ctx)
        headers = resolve(ts['headers'], ctx)
        data = resolve(ts['data'], ctx)
        expect = resolve(ts['expect'], ctx)

        response = requests.request(method=method,url=ctx['host']+url,params=params,headers=headers,json=data)
        resp['raw'] = response.text

        res = response.json()

        if match(res, expect):
            ctx |= res
            ctx |= resolve(ts['ctx'], ctx)
            return True, f"{url}"
        else:
            return False, f"{url} ressult:{res} != {expect}"
    except Exception as ex:
        return False, f"{url} exception: {ex.args}, response: {resp['raw']}"


# color text message
def red(s): return f"\033[91m {s}\033[00m"
def green(s): return f"\033[92m {s}\033[00m"
def yello(s): return f"\033[93m {s}\033[00m"
def purple(s): return f"\033[95m {s}\033[00m"
def cyan(s): return f"\033[96m {s}\033[00m"


# print test result
def print_result(idx, passed, info):
    if passed:
        print(green(f"{datetime.now()} ------ passed, {'%5d' % idx},  {info}"))
    else:
        print(red(f"{datetime.now()} !!!!!! failed, {'%5d' % idx},  {info}"))


# construct test infomation
def T(url,method='get',params={},headers={},data={},expect={},ctx={}):
    return {'url':url,'method':method,'params':params,'headers':headers,'data':data, 'expect':expect,'ctx':ctx}


# run all tests
def run(tests, ctx):
    idx = 0
    for ts in tests:
        if type(ts) is list:
            for t in ts:
                print_result(idx, *apply_single(t, ctx))
                idx += 1
        else:
            print_result(idx, *apply_single(ts, ctx))
            idx += 1
