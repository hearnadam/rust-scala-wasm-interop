import * as http from "request:http-client-stub/stub-http-client";

export const api = {
  gist(id) {
    const componentId = "e59bd348-23c7-47b8-95e4-a5f027f807f3"
    const workerId = "example-http-client-004"
    const httpClient = new http(`worker://${componentId}/${workerId}`)
    const url = `https://gist.githubusercontent.com/hearnadam/${id}/raw/${id}/`
    return httpClient.get(url);
  }
}
