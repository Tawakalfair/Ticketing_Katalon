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

response = WS.sendRequest(findTestObject('Roles/Get Roles'), FailureHandling.STOP_ON_FAILURE)

WS.verifyResponseStatusCode(response, 200)

countBefore = WS.getElementsCount(response, '')

response = WS.sendRequest(findTestObject('Roles/Delete Roles', [('id') : '1f87ee98-bcde-497d-a9a0-610435ea1ee7']))

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('Roles/Get Roles'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementsCount(response, '', countBefore-1)

