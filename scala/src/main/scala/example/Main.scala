package example

@cloud.golem.WitExport
object ComponentName extends Api { self =>

  def gist(id: String): WitResult[String, String] = {
    val url = s"https://gist.githubusercontent.com/hearnadam/$id/raw/$id/"
    WitResult.ok(id)
  }
}
