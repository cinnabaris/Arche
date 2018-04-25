import Cookies from 'js-cookie'
import antdEnUs from 'antd/lib/locale-provider/en_US'
import antdZhHant from 'antd/lib/locale-provider/zh_TW'
import dataEnUs from 'react-intl/locale-data/en'
import dataZhHans from 'react-intl/locale-data/zh'
const KEY = 'locale'

export const set = (lang) => {
  localStorage.setItem(KEY, lang)
  Cookies.set(KEY, lang, {
    expires: 365 * 100,
    path: '/'
  })
}

export const get = () => {
  var lang = localStorage.getItem(KEY) || Cookies.get(KEY)
  switch (lang) {
    case 'zh-Hans':
      return {locale: lang, data: dataZhHans, antd: null, moment: 'zh-cn'}
    case 'zh-Hant':
      return {locale: lang, data: dataZhHans, antd: antdZhHant, moment: 'zh-tw'}
    default:
      return {locale: 'en-US', data: dataEnUs, antd: antdEnUs, moment: 'en-gb'}
  }
}
