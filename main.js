async function translate(text, from, to, options) {
    const {  utils } = options;
    const { http } = utils;
    const { fetch, Body }=http;

    const form = new FormData();
    form.append("content", text);

    const res = await fetch(`http://res.d.hjfile.cn/v10/dict/translation/${from}/${to}`, {
        method: 'POST',
        headers:{
            "Host": "res.d.hjfile.cn",
            "Origin": "http://res.d.hjfile.cn",
            "Referer": "http://res.d.hjfile.cn/app/trans",
            "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
            "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
            "Cookie": "HJ_UID=390f25c7-c9f3-b237-f639-62bd23cd431f; HJC_USRC=uzhi; HJC_NUID=1",
        },
        body: Body.form(form)
    });

    if (res.ok) {
        let result = res.data;
        const { data } = result;
        if (data && data.content) {
            return data.content;
        } else {
            throw JSON.stringify(result.trim());
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}