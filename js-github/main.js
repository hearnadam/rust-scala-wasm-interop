import * as http from  "request:http-client-stub/stub-http-client";

export const api = {
    gist(id) {
        const workerId = "12344"
        const httpClient = new http(`worker://${workerId}/get`)
        const url = `https://gist.githubusercontent.com/hearnadam/${id}/raw/${id}/`
        return httpClient.get(url);
    }
}
