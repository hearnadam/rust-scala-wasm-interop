package example

@cloud.golem.WitExport
object ComponentName extends Api { self =>

  def gist(id: String): WitResult[String, String] = {
    println(s"Fetching gist id: $id")
    if (true)
      WitResult.ok(id)
    else
        WitResult.err("Error")
  }
}
