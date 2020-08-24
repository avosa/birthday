import datetime

from flask import Flask, render_template

app = Flask(__name__)

@app.route("/")
def index():
	month = datetime.date.today().month
	day = datetime.date.today().day

	return render_template("birthday.html", month=month, day=day)