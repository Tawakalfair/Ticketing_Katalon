import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequest(findTestObject('Client-Products/Put Client-Products', [('idProduct') : 'd420a688-e41f-474c-aed2-71bb0a3bea4b'
            , ('idCompany') : '8eac4724-c936-45d1-a4a7-080bf89b261a', ('id') : '590120e1-fea5-45c7-8504-9af14591e576', ('ticketUrgent') : 4
            , ('ticketMedium') : 5]))

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('Client-Products/Get Client-Products'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, '[0].id', id)

WS.verifyElementPropertyValue(response, '[0].idCompany.id', idCompany)

WS.verifyElementPropertyValue(response, '[0].idProduct.id', idProduct)

