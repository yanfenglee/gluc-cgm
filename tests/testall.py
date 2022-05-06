from ti import T, run
import numpy as np


ctx = {}
ctx['host'] = 'http://localhost:8899'

#ctx['username'] = f'testname2{np.random.randint(0,10000000, 1)}'
ctx['username'] = 'lyf34'
ctx['pswd'] = 'asdf'
ctx['postdata'] = [{'device': 'xDrip-LimiTTer','date': 1608046900101,'dateString': '2020-12-15T23:41:40.100+0800','sgv': 162,'delta': -0.353,'direction': 'Flat','type': 'sgv','filtered': 181647.0452,'unfiltered': 181647.0452,'rssi': 100,'noise': 1,'sysTime': '2020-12-15T23:41:40.100+0800','utcOffset': None,'slope': None,'intercept': None,'scale': None,'mbg': None,'created_time': None}]

ctx['data2'] = {'device': 'xDrip-LimiTTer','timestamp': 1651798118013}

tests = [
    T(url='/user/register',method='post',data={'username':'$.username','password':'$.pswd'}, expect={'code':'0'}),
    T(url='/user/login',method='post',data={'username':'$.username','password':'$.pswd'}, expect={'code':'0'},ctx={'login_token':'$.data.token'}),
    
    # [
    #     T(url='/api/v1/entries',method='post',headers={'token':'$.login_token'},data='$.postdata', expect={'code':'0'}),
    #     T(url='/api/v1/entries.json',params={'rr':9999999999999999,'count':1},method='get',headers={'token':'$.login_token'},expect={'code':'0'}),

    #     T(url='/api/v1/entries',method='post',headers={'api-secret':'$.login_token'},data='$.postdata', expect={'code':'0'}),
    #     T(url='/api/v1/entries.json',params={'rr':9999999999999999,'count':1},method='get',headers={'api-secret':'$.login_token'},expect={'code':'0'}),
    # ],


        #T(url='/api/v1/treatments',method='post',headers={'api-secret':'$.login_token'},data='$.data2', expect={'code':'0'}),
        #T(url='/api/v1/treatments',method='put',headers={'api-secret':'$.login_token'},data='$.data2', expect={'code':'0'}),

        T(url='/api/v1/treatments',method='get',headers={'api-secret':'$.login_token'}, expect={'code':'0'}),

        # T(url='/api/v1/devicestatus',method='post',headers={'api-secret':'$.login_token'},data='$.data2', expect={'code':'0'}),

        # T(url='/api/v1/status.json',params={'rr':9999999999999999,'count':1},method='get',headers={'api-secret':'$.login_token'},expect={'code':'0'}),

        # T(url='/api/v1/activity',method='post',headers={'api-secret':'$.login_token'},data='$.data2', expect={'code':'0'}),
        # T(url='/api/v1/profile',method='post',headers={'api-secret':'$.login_token'},data='$.data2', expect={'code':'0'}),
        # T(url='/api/v1/profile',method='get',headers={'api-secret':'$.login_token'},data='$.data2', expect={'code':'0'}),

]

run(tests, ctx, verbose=True)