FROM python:3.8-slim-buster

WORKDIR /app

COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements_gcp.txt

COPY . .

EXPOSE 8080

CMD [ "python", "./app.py" ]