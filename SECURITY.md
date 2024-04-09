# Security Announcements

Join the [OpenLLaMA Zulip](https://openllama.zulipchat.com/#narrow/stream/432302-general/topic/security) stream for informations about security and major announcements.

## Report a Vulnerability

We’re extremely grateful for security researchers and users that report vulnerabilities to the OpenLLaMA Community. All reports are thoroughly investigated by a dedicated committee of community volunteers called [Product Security Committee](./security/security-release-process.md#product-security-committee).

To make a report, please email the private [security email receiver](mailto:general.3de16ce3badd55db94780998fb730993.show-sender@streams.zulipchat.com) with the security details and the details expected for [all OpenLLaMA bug reports](https://github.com/ComputeIO/OpenLLaMA/blob/master/CONTRIBUTING.md#bug-reports).Note: (Groups.io requires users to subscribe to relevant email groups before sending emails. This is to ensure that only group members can send and receive emails within the group, maintaining the security and privacy of the group. Therefore, according to Groups.io's regulations, you need to subscribe to an email group before being able to send emails)

### When Should I Report a Vulnerability?

- When discovered a potential security vulnerability in OpenLLaMA 
- When unsure how a vulnerability affects OpenLLaMA
- When discovered a vulnerability in another project that OpenLLaMA depends on

### When Should I NOT Report a Vulnerability?

- Need help tuning OpenLLaMA for security
- Need help applying security related updates
- When an issue is not security related

## Security Vulnerability Response

Each report is acknowledged and analyzed by Product Security Committee members within 3 working days. This will set off the [Security Release Process](./security/security-release-process.md).

Any vulnerability information shared with Product Security Committee stays within OpenLLaMA project and will not be disseminated to other projects unless it is necessary to get the issue fixed.

As the security issue moves from triage, to identified fix, to release planning we will keep the reporter updated.

## Public Disclosure Timing

A public disclosure date is negotiated by the CubeFS Product Security Committee and the bug reporter. We prefer to fully disclose the bug as soon as possible once user mitigation is available. It is reasonable to delay disclosure when the bug or the fix is not yet fully understood, the solution is not well-tested, or for vendor coordination. The timeframe for disclosure is from immediate (especially if it's already publicly known) to a few weeks. As a basic default, we expect report date to disclosure date to be on the order of 7 days. The CubeFS Product Security Committee holds the final say when setting a disclosure date.
