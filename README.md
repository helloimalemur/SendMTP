# SendMTP

SendMTP \
 --smtp-server "smtp-relay.gmail.com" \
 --smtp-port "587" \
 --smtp-username "<username>" \
 --smtp-password "<password>" \
 --from-email "noreply <noreply@domain.com>" \
 --to-email "default@your.pagerduty.com" \
 --email-subject "$SERVICEDISPLAYNAME$ is $SERVICESTATE$" \
 --email-body "$SERVICEDISPLAYNAME$ ($SERVICEDESC$) is $SERVICESTATE$"
