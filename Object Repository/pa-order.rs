<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>pa-order</name>
   <tag></tag>
   <elementGuidId>db3d5888-73c6-4cf1-b418-fb2d4c8ff530</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;OrderDate\&quot;: \&quot;${order_date}\&quot;,\n  \&quot;MaskapaiID\&quot;: \&quot;${maskapai_id}\&quot;,\n  \&quot;Page\&quot;: \&quot;${page}\&quot;,\n  \&quot;RowsPerPage\&quot;: \&quot;${row}\&quot;,\n  \&quot;Source\&quot;: \&quot;${source}\&quot;\n}&quot;,
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
   <restUrl>https://api-dev.bcaf.id:8445/personal-accident-insurance/order</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'VXsh7jhEGrnJhGfiyt3QB5Yk6SWl'</defaultValue>
      <description></description>
      <id>541d84ea-f8a4-4274-b756-c771be5841b4</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'2021-09-26'</defaultValue>
      <description></description>
      <id>7afd5c43-58e8-466f-9ff1-79cb8b255ee8</id>
      <masked>false</masked>
      <name>order_date</name>
   </variables>
   <variables>
      <defaultValue>'PA_SIMAS'</defaultValue>
      <description></description>
      <id>be34a6cf-1e42-4ad2-a740-a3d6d4b99ff6</id>
      <masked>false</masked>
      <name>maskapai_id</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>95ad650b-4b67-4bac-a7ab-e278bf398312</id>
      <masked>false</masked>
      <name>page</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>0ea13860-7fa4-43b1-906f-c2eb49ae2ba0</id>
      <masked>false</masked>
      <name>row</name>
   </variables>
   <variables>
      <defaultValue>'SIMAS'</defaultValue>
      <description></description>
      <id>356c4d51-2134-43a4-bb20-5e77ea285129</id>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'Data.BelumTerimaPolis[0].Konsumen.TempatLahir', '')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
