<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>pa-feedback</name>
   <tag></tag>
   <elementGuidId>4e514273-c5cc-4af9-8617-901d86c0b603</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;MaskapaiID\&quot;: \&quot;${maskapai_id}\&quot;,\n  \&quot;OrderID\&quot;: \&quot;${order_id}\&quot;,\n  \&quot;NoPolis\&quot;: \&quot;${no_polis}\&quot;,\n  \&quot;Base64EPolis\&quot;: \&quot;${base64}\&quot;,\n  \&quot;Source\&quot;: \&quot;${source}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api-dev.bcaf.id:8445/personal-accident-insurance/feedback</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'6sSYpmqLtSL8pZVqLtGcLZ5p122S'</defaultValue>
      <description></description>
      <id>cf200c50-b41b-40ac-8f20-1de1e9395375</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'PA_SIMAS'</defaultValue>
      <description></description>
      <id>4058e6c9-12e8-41e7-8d13-ff48fc79b302</id>
      <masked>false</masked>
      <name>maskapai_id</name>
   </variables>
   <variables>
      <defaultValue>'29D947AC-1F9A-4871-807A-0008CA1B533B'</defaultValue>
      <description></description>
      <id>52827903-045d-4b04-a1eb-50ebdaa07e55</id>
      <masked>false</masked>
      <name>order_id</name>
   </variables>
   <variables>
      <defaultValue>'POLIS-SIMAS-PA-005'</defaultValue>
      <description></description>
      <id>a797b85a-d532-42d6-ae38-d3287d7ccc40</id>
      <masked>false</masked>
      <name>no_polis</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>fa562ff9-31ac-4cc6-87db-7f950aadbc6b</id>
      <masked>false</masked>
      <name>base64</name>
   </variables>
   <variables>
      <defaultValue>'SIMAS'</defaultValue>
      <description></description>
      <id>6a2ceeeb-d537-4ecf-a606-f19c198faa90</id>
      <masked>false</masked>
      <name>source</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
