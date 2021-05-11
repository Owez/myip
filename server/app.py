from flask import Flask, render_template, request, send_from_directory

SCRIPT = "curl something here ig"

app = Flask(__name__)


@app.route("/")
def index():
    return render_template("index.html", addr=request.remote_addr, script=SCRIPT)


@app.route("/download")
def download():
    return send_from_directory("static", filename="myip")


@app.route("/get")
def api_ipv4():
    return request.remote_addr


if __name__ == "__main__":
    app.run(debug=True)
