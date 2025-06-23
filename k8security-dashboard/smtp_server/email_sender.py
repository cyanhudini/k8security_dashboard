import smtplib
import psycopg2
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

def send_email(report, receiver_emails):
    sender_email = ""
    subject = "High Severity Vulnerabilities im Kubernetes CLuster"

    msg = MIMEMultipart()
    msg['From'] = sender_email
    msg['To'] = ", ".join(receiver_emails)
    msg['Subject'] = subject

    body = "Im Anhang findest du eine Zusammenfassung der durch Trivy gefunden Vulnerabilities mit einer Schwere höher HIGH."
    msg.attach(MIMEText(body, 'plain'))

    attachment = MIMEText(report, 'plain')
    attachment.add_header('Content-Disposition', 'attachment', filename="report.json")
    msg.attach(attachment)
    # env
    server = smtplib.SMTP_SSL('')
    server.login("", "")
    server.sendmail(sender_email, receiver_emails, msg.as_string())
    server.quit()

def get_active_emails():
    conn = psycopg2.connect(
        dbname="",
        user="",
        password="",
        host="",
        port=""
)
    cur = conn.cursor()
    cur.execute("SELECT email_adress FROM emails WHERE receiving = true")
    emails = [row[0] for row in cur.fetchall()]
    cur.close()
    conn.close()
    return emails

def main():
    with open("report.json", "r") as f:
        report_content = f.read()
    if report_content:
        receivers = get_active_emails()
        if receivers:
            send_email(report_content, receivers)
        else:
            print("No active emails found.")
    else:
        
        print("Keine zu sendende Datei gefunden")

if __name__ == "__main__":
    main()
