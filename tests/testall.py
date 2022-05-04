from ti import T, run
import numpy as np


ctx = {}
ctx['host'] = 'http://127.0.0.1:8899'
ctx['username'] = f'testname{np.random.randint(0,10000000, 1)}'
ctx['postdata'] = [{'device': 'xDrip-LimiTTer','date': 1608046900100,'date_str': '2020-12-15T23:41:40.100+0800','sgv': 162,'delta': -0.353,'direction': 'Flat','type': 'sgv','filtered': 181647.0452,'unfiltered': 181647.0452,'rssi': 100,'noise': 1,'sys_time': '2020-12-15T23:41:40.100+0800','utc_offset': None,'slope': None,'intercept': None,'scale': None,'mbg': None,'created_time': None}]

tests = [
    T(url='/user/register',method='post',data={'username':'$.username','password':'123456'}, expect={'code':'0'}),
    T(url='/user/login',method='post',data={'username':'$.username','password':'123456'}, expect={'code':'0'},ctx={'login_token':'$.data.token'}),

    T(url='/api/v1/entries',method='post',headers={'token':'$.login_token'},data='$.postdata', expect={'code':'0'}),
    T(url='/api/v1/entries.json',params={'rr':9999999999999999,'count':3},method='get',headers={'token':'$.login_token'},expect={'code':'0'}),

]

run(tests, ctx, verbose=True)