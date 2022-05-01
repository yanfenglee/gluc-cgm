from ti import T, run


ctx = {}
ctx['host'] = 'http://127.0.0.1:8899'
ctx['username'] = 'testname132'
ctx['postdata'] = {}

tests = [
    T(url='/user/register',method='post',data={'username':'$.username','password':'123456'}, expect={'code':'0'}),
    T(url='/user/login',method='post',data={'username':'$.username','password':'123456'}, expect={'code':'0'},ctx={'login_token':'$.data.token'}),

    [
        T(url='/api/v1/entries',method='post',headers={'token':'$.login_token'},data='$.postdata', expect={'code':'0'}),
        T(url='/api/v1/entries.json?rr=999999999999999&count=10',method='get',headers={'token':'$.login_token'},expect={'code':'0'}),
    ]*3
]

run(tests, ctx)