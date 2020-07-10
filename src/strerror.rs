#![allow(non_camel_case_types,unused_assignments,unused_unsafe,non_snake_case,non_upper_case_globals,unused_must_use)]

use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
/* * \ingroup libusb_misc
 * Error codes. Most libusb functions return 0 on success or one of these
 * codes on failure.
 * You can call libusb_error_name() to retrieve a string representation of an
 * error code or libusb_strerror() to get an end-user suitable description of
 * an error code.
 */
pub type libusb_error = libc::c_int;
/* NB: Remember to update LIBUSB_ERROR_COUNT below as well as the
message strings in strerror.c when adding new error codes here. */
/* * Other error */
pub const LIBUSB_ERROR_OTHER: libusb_error = -99;
/* * Operation not supported or unimplemented on this platform */
pub const LIBUSB_ERROR_NOT_SUPPORTED: libusb_error = -12;
/* * Insufficient memory */
pub const LIBUSB_ERROR_NO_MEM: libusb_error = -11;
/* * System call interrupted (perhaps due to signal) */
pub const LIBUSB_ERROR_INTERRUPTED: libusb_error = -10;
/* * Pipe error */
pub const LIBUSB_ERROR_PIPE: libusb_error = -9;
/* * Overflow */
pub const LIBUSB_ERROR_OVERFLOW: libusb_error = -8;
/* * Operation timed out */
pub const LIBUSB_ERROR_TIMEOUT: libusb_error = -7;
/* * Resource busy */
pub const LIBUSB_ERROR_BUSY: libusb_error = -6;
/* * Entity not found */
pub const LIBUSB_ERROR_NOT_FOUND: libusb_error = -5;
/* * No such device (it may have been disconnected) */
pub const LIBUSB_ERROR_NO_DEVICE: libusb_error = -4;
/* * Access denied (insufficient permissions) */
pub const LIBUSB_ERROR_ACCESS: libusb_error = -3;
/* * Invalid parameter */
pub const LIBUSB_ERROR_INVALID_PARAM: libusb_error = -2;
/* * Input/output error */
pub const LIBUSB_ERROR_IO: libusb_error = -1;
/* * Success (no error) */
pub const LIBUSB_SUCCESS: libusb_error = 0;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
/*
 * libusb strerror code
 * Copyright © 2013 Hans de Goede <hdegoede@redhat.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */
/* * \ingroup libusb_misc
 * How to add a new \ref libusb_strerror() translation:
 * <ol>
 * <li> Download the latest \c strerror.c from:<br>
 *      https://raw.github.com/libusb/libusb/master/libusb/strerror.c </li>
 * <li> Open the file in an UTF-8 capable editor </li>
 * <li> Add the 2 letter <a href="http://en.wikipedia.org/wiki/List_of_ISO_639-1_codes">ISO 639-1</a>
 *      code for your locale at the end of \c usbi_locale_supported[]<br>
 *    Eg. for Chinese, you would add "zh" so that:
 *    \code... usbi_locale_supported[] = { "en", "nl", "fr" };\endcode
 *    becomes:
 *    \code... usbi_locale_supported[] = { "en", "nl", "fr", "zh" };\endcode </li>
 * <li> Copy the <tt>{ / * English (en) * / ... }</tt> section and add it at the end of \c usbi_localized_errors<br>
 *    Eg. for Chinese, the last section of \c usbi_localized_errors could look like:
 *    \code
 *     }, { / * Chinese (zh) * /
 *         "Success",
 *         ...
 *         "Other error",
 *     },
 * };\endcode </li>
 * <li> Translate each of the English messages from the section you copied into your language </li>
 * <li> Save the file (in UTF-8 format) and send it to \c libusb-devel\@lists.sourceforge.net </li>
 * </ol>
 */
static mut usbi_locale_supported: [*const libc::c_char; 6] = [
    b"en\x00" as *const u8 as *const libc::c_char,
    b"nl\x00" as *const u8 as *const libc::c_char,
    b"fr\x00" as *const u8 as *const libc::c_char,
    b"ru\x00" as *const u8 as *const libc::c_char,
    b"de\x00" as *const u8 as *const libc::c_char,
    b"hu\x00" as *const u8 as *const libc::c_char,
];
static mut usbi_localized_errors: [[*const libc::c_char; 14]; 6] =
    [[b"Success\x00" as *const u8 as *const libc::c_char,
      b"Input/Output Error\x00" as *const u8 as *const libc::c_char,
      b"Invalid parameter\x00" as *const u8 as *const libc::c_char,
      b"Access denied (insufficient permissions)\x00" as *const u8 as
          *const libc::c_char,
      b"No such device (it may have been disconnected)\x00" as *const u8 as
          *const libc::c_char,
      b"Entity not found\x00" as *const u8 as *const libc::c_char,
      b"Resource busy\x00" as *const u8 as *const libc::c_char,
      b"Operation timed out\x00" as *const u8 as *const libc::c_char,
      b"Overflow\x00" as *const u8 as *const libc::c_char,
      b"Pipe error\x00" as *const u8 as *const libc::c_char,
      b"System call interrupted (perhaps due to signal)\x00" as *const u8 as
          *const libc::c_char,
      b"Insufficient memory\x00" as *const u8 as *const libc::c_char,
      b"Operation not supported or unimplemented on this platform\x00" as
          *const u8 as *const libc::c_char,
      b"Other error\x00" as *const u8 as *const libc::c_char],
     [b"Gelukt\x00" as *const u8 as *const libc::c_char,
      b"Invoer-/uitvoerfout\x00" as *const u8 as *const libc::c_char,
      b"Ongeldig argument\x00" as *const u8 as *const libc::c_char,
      b"Toegang geweigerd (onvoldoende toegangsrechten)\x00" as *const u8 as
          *const libc::c_char,
      b"Apparaat bestaat niet (verbinding met apparaat verbroken?)\x00" as
          *const u8 as *const libc::c_char,
      b"Niet gevonden\x00" as *const u8 as *const libc::c_char,
      b"Apparaat of hulpbron is bezig\x00" as *const u8 as
          *const libc::c_char,
      b"Bewerking verlopen\x00" as *const u8 as *const libc::c_char,
      b"Waarde is te groot\x00" as *const u8 as *const libc::c_char,
      b"Gebroken pijp\x00" as *const u8 as *const libc::c_char,
      b"Onderbroken systeemaanroep\x00" as *const u8 as *const libc::c_char,
      b"Onvoldoende geheugen beschikbaar\x00" as *const u8 as
          *const libc::c_char,
      b"Bewerking wordt niet ondersteund\x00" as *const u8 as
          *const libc::c_char,
      b"Andere fout\x00" as *const u8 as *const libc::c_char],
     [b"Succ\xc3\xa8s\x00" as *const u8 as *const libc::c_char,
      b"Erreur d\'entr\xc3\xa9e/sortie\x00" as *const u8 as
          *const libc::c_char,
      b"Param\xc3\xa8tre invalide\x00" as *const u8 as *const libc::c_char,
      b"Acc\xc3\xa8s refus\xc3\xa9 (permissions insuffisantes)\x00" as
          *const u8 as *const libc::c_char,
      b"P\xc3\xa9riph\xc3\xa9rique introuvable (peut-\xc3\xaatre d\xc3\xa9connect\xc3\xa9)\x00"
          as *const u8 as *const libc::c_char,
      b"El\xc3\xa9ment introuvable\x00" as *const u8 as *const libc::c_char,
      b"Resource d\xc3\xa9j\xc3\xa0 occup\xc3\xa9e\x00" as *const u8 as
          *const libc::c_char,
      b"Operation expir\xc3\xa9e\x00" as *const u8 as *const libc::c_char,
      b"D\xc3\xa9bordement\x00" as *const u8 as *const libc::c_char,
      b"Erreur de pipe\x00" as *const u8 as *const libc::c_char,
      b"Appel syst\xc3\xa8me abandonn\xc3\xa9 (peut-\xc3\xaatre \xc3\xa0 cause d\xe2\x80\x99un signal)\x00"
          as *const u8 as *const libc::c_char,
      b"M\xc3\xa9moire insuffisante\x00" as *const u8 as *const libc::c_char,
      b"Op\xc3\xa9ration non support\xc3\xa9e or non impl\xc3\xa9ment\xc3\xa9e sur cette plateforme\x00"
          as *const u8 as *const libc::c_char,
      b"Autre erreur\x00" as *const u8 as *const libc::c_char],
     [b"\xd0\xa3\xd1\x81\xd0\xbf\xd0\xb5\xd1\x85\x00" as *const u8 as
          *const libc::c_char,
      b"\xd0\x9e\xd1\x88\xd0\xb8\xd0\xb1\xd0\xba\xd0\xb0 \xd0\xb2\xd0\xb2\xd0\xbe\xd0\xb4\xd0\xb0/\xd0\xb2\xd1\x8b\xd0\xb2\xd0\xbe\xd0\xb4\xd0\xb0\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\x9d\xd0\xb5\xd0\xb2\xd0\xb5\xd1\x80\xd0\xbd\xd1\x8b\xd0\xb9 \xd0\xbf\xd0\xb0\xd1\x80\xd0\xb0\xd0\xbc\xd0\xb5\xd1\x82\xd1\x80\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\x94\xd0\xbe\xd1\x81\xd1\x82\xd1\x83\xd0\xbf \xd0\xb7\xd0\xb0\xd0\xbf\xd1\x80\xd0\xb5\xd1\x89\xd1\x91\xd0\xbd (\xd0\xbd\xd0\xb5 \xd1\x85\xd0\xb2\xd0\xb0\xd1\x82\xd0\xb0\xd0\xb5\xd1\x82 \xd0\xbf\xd1\x80\xd0\xb0\xd0\xb2)\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\xa3\xd1\x81\xd1\x82\xd1\x80\xd0\xbe\xd0\xb9\xd1\x81\xd1\x82\xd0\xb2\xd0\xbe \xd0\xbe\xd1\x82\xd1\x81\xd1\x83\xd1\x82\xd1\x81\xd1\x82\xd0\xb2\xd1\x83\xd0\xb5\xd1\x82 (\xd0\xb2\xd0\xbe\xd0\xb7\xd0\xbc\xd0\xbe\xd0\xb6\xd0\xbd\xd0\xbe, \xd0\xbe\xd0\xbd\xd0\xbe \xd0\xb1\xd1\x8b\xd0\xbb\xd0\xbe \xd0\xbe\xd1\x82\xd1\x81\xd0\xbe\xd0\xb5\xd0\xb4\xd0\xb8\xd0\xbd\xd0\xb5\xd0\xbd\xd0\xbe)\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\xad\xd0\xbb\xd0\xb5\xd0\xbc\xd0\xb5\xd0\xbd\xd1\x82 \xd0\xbd\xd0\xb5 \xd0\xbd\xd0\xb0\xd0\xb9\xd0\xb4\xd0\xb5\xd0\xbd\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\xa0\xd0\xb5\xd1\x81\xd1\x83\xd1\x80\xd1\x81 \xd0\xb7\xd0\xb0\xd0\xbd\xd1\x8f\xd1\x82\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\x98\xd1\x81\xd1\x82\xd0\xb5\xd0\xba\xd0\xbb\xd0\xbe \xd0\xb2\xd1\x80\xd0\xb5\xd0\xbc\xd1\x8f \xd0\xbe\xd0\xb6\xd0\xb8\xd0\xb4\xd0\xb0\xd0\xbd\xd0\xb8\xd1\x8f \xd0\xbe\xd0\xbf\xd0\xb5\xd1\x80\xd0\xb0\xd1\x86\xd0\xb8\xd0\xb8\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\x9f\xd0\xb5\xd1\x80\xd0\xb5\xd0\xbf\xd0\xbe\xd0\xbb\xd0\xbd\xd0\xb5\xd0\xbd\xd0\xb8\xd0\xb5\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\x9e\xd1\x88\xd0\xb8\xd0\xb1\xd0\xba\xd0\xb0 \xd0\xba\xd0\xb0\xd0\xbd\xd0\xb0\xd0\xbb\xd0\xb0\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\xa1\xd0\xb8\xd1\x81\xd1\x82\xd0\xb5\xd0\xbc\xd0\xbd\xd1\x8b\xd0\xb9 \xd0\xb2\xd1\x8b\xd0\xb7\xd0\xbe\xd0\xb2 \xd0\xbf\xd1\x80\xd0\xb5\xd1\x80\xd0\xb2\xd0\xb0\xd0\xbd (\xd0\xb2\xd0\xbe\xd0\xb7\xd0\xbc\xd0\xbe\xd0\xb6\xd0\xbd\xd0\xbe, \xd1\x81\xd0\xb8\xd0\xb3\xd0\xbd\xd0\xb0\xd0\xbb\xd0\xbe\xd0\xbc)\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\x9f\xd0\xb0\xd0\xbc\xd1\x8f\xd1\x82\xd1\x8c \xd0\xb8\xd1\x81\xd1\x87\xd0\xb5\xd1\x80\xd0\xbf\xd0\xb0\xd0\xbd\xd0\xb0\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\x9e\xd0\xbf\xd0\xb5\xd1\x80\xd0\xb0\xd1\x86\xd0\xb8\xd1\x8f \xd0\xbd\xd0\xb5 \xd0\xbf\xd0\xbe\xd0\xb4\xd0\xb4\xd0\xb5\xd1\x80\xd0\xb6\xd0\xb8\xd0\xb2\xd0\xb0\xd0\xb5\xd1\x82\xd1\x81\xd1\x8f \xd0\xb4\xd0\xb0\xd0\xbd\xd0\xbd\xd0\xbe\xd0\xb9 \xd0\xbf\xd0\xbb\xd0\xb0\xd1\x82\xd1\x84\xd0\xbe\xd1\x80\xd0\xbc\xd0\xbe\xd0\xb9\x00"
          as *const u8 as *const libc::c_char,
      b"\xd0\x9d\xd0\xb5\xd0\xb8\xd0\xb7\xd0\xb2\xd0\xb5\xd1\x81\xd1\x82\xd0\xbd\xd0\xb0\xd1\x8f \xd0\xbe\xd1\x88\xd0\xb8\xd0\xb1\xd0\xba\xd0\xb0\x00"
          as *const u8 as *const libc::c_char],
     [b"Erfolgreich\x00" as *const u8 as *const libc::c_char,
      b"Eingabe-/Ausgabefehler\x00" as *const u8 as *const libc::c_char,
      b"Ung\xc3\xbcltiger Parameter\x00" as *const u8 as *const libc::c_char,
      b"Keine Berechtigung (Zugriffsrechte fehlen)\x00" as *const u8 as
          *const libc::c_char,
      b"Kein passendes Ger\xc3\xa4t gefunden (es k\xc3\xb6nnte entfernt worden sein)\x00"
          as *const u8 as *const libc::c_char,
      b"Entit\xc3\xa4t nicht gefunden\x00" as *const u8 as
          *const libc::c_char,
      b"Die Ressource ist belegt\x00" as *const u8 as *const libc::c_char,
      b"Die Wartezeit f\xc3\xbcr die Operation ist abgelaufen\x00" as
          *const u8 as *const libc::c_char,
      b"Mehr Daten empfangen als erwartet\x00" as *const u8 as
          *const libc::c_char,
      b"Daten\xc3\xbcbergabe unterbrochen (broken pipe)\x00" as *const u8 as
          *const libc::c_char,
      b"Unterbrechung w\xc3\xa4hrend des Betriebssystemaufrufs\x00" as
          *const u8 as *const libc::c_char,
      b"Nicht gen\xc3\xbcgend Hauptspeicher verf\xc3\xbcgbar\x00" as *const u8
          as *const libc::c_char,
      b"Die Operation wird nicht unterst\xc3\xbctzt oder ist auf dieser Platform nicht implementiert\x00"
          as *const u8 as *const libc::c_char,
      b"Allgemeiner Fehler\x00" as *const u8 as *const libc::c_char],
     [b"Sikeres\x00" as *const u8 as *const libc::c_char,
      b"Be-/kimeneti hiba\x00" as *const u8 as *const libc::c_char,
      b"\xc3\x89rv\xc3\xa9nytelen param\xc3\xa9ter\x00" as *const u8 as
          *const libc::c_char,
      b"Hozz\xc3\xa1f\xc3\xa9r\xc3\xa9s megtagadva\x00" as *const u8 as
          *const libc::c_char,
      b"Az eszk\xc3\xb6z nem tal\xc3\xa1lhat\xc3\xb3 (elt\xc3\xa1vol\xc3\xadtott\xc3\xa1k?)\x00"
          as *const u8 as *const libc::c_char,
      b"Nem tal\xc3\xa1lhat\xc3\xb3\x00" as *const u8 as *const libc::c_char,
      b"Az er\xc5\x91forr\xc3\xa1s foglalt\x00" as *const u8 as
          *const libc::c_char,
      b"Id\xc5\x91t\xc3\xball\xc3\xa9p\xc3\xa9s\x00" as *const u8 as
          *const libc::c_char,
      b"T\xc3\xbalcsordul\xc3\xa1s\x00" as *const u8 as *const libc::c_char,
      b"T\xc3\xb6r\xc3\xb6tt adatcsatorna\x00" as *const u8 as
          *const libc::c_char,
      b"Rendszerh\xc3\xadv\xc3\xa1s megszak\xc3\xadtva\x00" as *const u8 as
          *const libc::c_char,
      b"Nincs el\xc3\xa9g mem\xc3\xb3ria\x00" as *const u8 as
          *const libc::c_char,
      b"A m\xc5\xb1velet nem t\xc3\xa1mogatott ezen a rendszeren\x00" as
          *const u8 as *const libc::c_char,
      b"\xc3\x81ltal\xc3\xa1nos hiba\x00" as *const u8 as
          *const libc::c_char]];
// Initialized in run_static_initializers
static mut usbi_error_strings: *const [*const libc::c_char; 14] =
    0 as *const [*const libc::c_char; 14];
/* * \ingroup libusb_misc
 * Set the language, and only the language, not the encoding! used for
 * translatable libusb messages.
 *
 * This takes a locale string in the default setlocale format: lang[-region]
 * or lang[_country_region][.codeset]. Only the lang part of the string is
 * used, and only 2 letter ISO 639-1 codes are accepted for it, such as "de".
 * The optional region, country_region or codeset parts are ignored. This
 * means that functions which return translatable strings will NOT honor the
 * specified encoding.
 * All strings returned are encoded as UTF-8 strings.
 *
 * If libusb_setlocale() is not called, all messages will be in English.
 *
 * The following functions return translatable strings: libusb_strerror().
 * Note that the libusb log messages controlled through libusb_set_debug()
 * are not translated, they are always in English.
 *
 * For POSIX UTF-8 environments if you want libusb to follow the standard
 * locale settings, call libusb_setlocale(setlocale(LC_MESSAGES, NULL)),
 * after your app has done its locale setup.
 *
 * \param locale locale-string in the form of lang[_country_region][.codeset]
 * or lang[-region], where lang is a 2 letter ISO 639-1 code
 * \returns LIBUSB_SUCCESS on success
 * \returns LIBUSB_ERROR_INVALID_PARAM if the locale doesn't meet the requirements
 * \returns LIBUSB_ERROR_NOT_FOUND if the requested language is not supported
 * \returns a LIBUSB_ERROR code on other errors
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_setlocale(locale: *const libc::c_char) -> libc::c_int {
    let mut i: size_t = 0;
    if locale.is_null()
        || strlen(locale) < 2 as libc::c_int as libc::c_ulong
        || *locale.offset(2 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
            && *locale.offset(2 as libc::c_int as isize) as libc::c_int != '-' as i32
            && *locale.offset(2 as libc::c_int as isize) as libc::c_int != '_' as i32
            && *locale.offset(2 as libc::c_int as isize) as libc::c_int != '.' as i32
    {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[*const libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        if *usbi_locale_supported[i as usize].offset(0 as libc::c_int as isize) as libc::c_int
            == ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *locale.offset(0 as libc::c_int as isize)
                            as libc::c_uchar
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(*locale.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int)
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*locale.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize)
                }
                __res
            })
            && *usbi_locale_supported[i as usize].offset(1 as libc::c_int as isize) as libc::c_int
                == ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *locale.offset(1 as libc::c_int as isize)
                                as libc::c_uchar
                                as libc::c_int;
                            __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            }
                        } else {
                            __res = tolower(*locale.offset(1 as libc::c_int as isize)
                                as libc::c_uchar
                                as libc::c_int)
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*locale.offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize)
                    }
                    __res
                })
        {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i == (::std::mem::size_of::<[*const libc::c_char; 6]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
    }
    usbi_error_strings =
        &*usbi_localized_errors.as_ptr().offset(i as isize) as *const [*const libc::c_char; 14];
    return LIBUSB_SUCCESS as libc::c_int;
}
/* * \ingroup libusb_misc
 * Returns a constant string with a short description of the given error code,
 * this description is intended for displaying to the end user and will be in
 * the language set by libusb_setlocale().
 *
 * The returned string is encoded in UTF-8.
 *
 * The messages always start with a capital letter and end without any dot.
 * The caller must not free() the returned string.
 *
 * \param errcode the error code whose description is desired
 * \returns a short description of the error code in UTF-8 encoding
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_strerror(errcode: libc::c_int) -> *const libc::c_char {
    let mut errcode_index: libc::c_int = -errcode;
    if errcode_index < 0 as libc::c_int || errcode_index >= 14 as libc::c_int {
        /* "Other Error", which should always be our last message, is returned */
        errcode_index = 14 as libc::c_int - 1 as libc::c_int
    }
    return (*usbi_error_strings)[errcode_index as usize];
}
unsafe extern "C" fn run_static_initializers() {
    usbi_error_strings = &*usbi_localized_errors
        .as_ptr()
        .offset(0 as libc::c_int as isize)
        as *const [*const libc::c_char; 14]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
