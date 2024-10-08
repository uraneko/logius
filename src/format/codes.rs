pub fn code(byte: u16) -> &'static str {
    match byte {
0 => stringify!(KEY_RESERVED),
1 => stringify!(KEY_ESC),
2 => stringify!(KEY_1),
3 => stringify!(KEY_2),
4 => stringify!(KEY_3),
5 => stringify!(KEY_4),
6 => stringify!(KEY_5),
7 => stringify!(KEY_6),
8 => stringify!(KEY_7),
9 => stringify!(KEY_8),
10 => stringify!(KEY_9),
11 => stringify!(KEY_0),
12 => stringify!(KEY_MINUS),
13 => stringify!(KEY_EQUAL),
14 => stringify!(KEY_BACKSPACE),
15 => stringify!(KEY_TAB),
16 => stringify!(KEY_Q),
17 => stringify!(KEY_W),
18 => stringify!(KEY_E),
19 => stringify!(KEY_R),
20 => stringify!(KEY_T),
21 => stringify!(KEY_Y),
22 => stringify!(KEY_U),
23 => stringify!(KEY_I),
24 => stringify!(KEY_O),
25 => stringify!(KEY_P),
26 => stringify!(KEY_LEFTBRACE),
27 => stringify!(KEY_RIGHTBRACE),
28 => stringify!(KEY_ENTER),
29 => stringify!(KEY_LEFTCTRL),
30 => stringify!(KEY_A),
31 => stringify!(KEY_S),
32 => stringify!(KEY_D),
33 => stringify!(KEY_F),
34 => stringify!(KEY_G),
35 => stringify!(KEY_H),
36 => stringify!(KEY_J),
37 => stringify!(KEY_K),
38 => stringify!(KEY_L),
39 => stringify!(KEY_SEMICOLON),
40 => stringify!(KEY_APOSTROPHE),
41 => stringify!(KEY_GRAVE),
42 => stringify!(KEY_LEFTSHIFT),
43 => stringify!(KEY_BACKSLASH),
44 => stringify!(KEY_Z),
45 => stringify!(KEY_X),
46 => stringify!(KEY_C),
47 => stringify!(KEY_V),
48 => stringify!(KEY_B),
49 => stringify!(KEY_N),
50 => stringify!(KEY_M),
51 => stringify!(KEY_COMMA),
52 => stringify!(KEY_DOT),
53 => stringify!(KEY_SLASH),
54 => stringify!(KEY_RIGHTSHIFT),
55 => stringify!(KEY_KPASTERISK),
56 => stringify!(KEY_LEFTALT),
57 => stringify!(KEY_SPACE),
58 => stringify!(KEY_CAPSLOCK),
59 => stringify!(KEY_F1),
60 => stringify!(KEY_F2),
61 => stringify!(KEY_F3),
62 => stringify!(KEY_F4),
63 => stringify!(KEY_F5),
64 => stringify!(KEY_F6),
65 => stringify!(KEY_F7),
66 => stringify!(KEY_F8),
67 => stringify!(KEY_F9),
68 => stringify!(KEY_F10),
69 => stringify!(KEY_NUMLOCK),
70 => stringify!(KEY_SCROLLLOCK),
71 => stringify!(KEY_KP7),
72 => stringify!(KEY_KP8),
73 => stringify!(KEY_KP9),
74 => stringify!(KEY_KPMINUS),
75 => stringify!(KEY_KP4),
76 => stringify!(KEY_KP5),
77 => stringify!(KEY_KP6),
78 => stringify!(KEY_KPPLUS),
79 => stringify!(KEY_KP1),
80 => stringify!(KEY_KP2),
81 => stringify!(KEY_KP3),
82 => stringify!(KEY_KP0),
83 => stringify!(KEY_KPDOT),

85 => stringify!(KEY_ZENKAKUHANKAKU),
86 => stringify!(KEY_102ND),
87 => stringify!(KEY_F11),
88 => stringify!(KEY_F12),
89 => stringify!(KEY_RO),
90 => stringify!(KEY_KATAKANA),
91 => stringify!(KEY_HIRAGANA),
92 => stringify!(KEY_HENKAN),
93 => stringify!(KEY_KATAKANAHIRAGANA),
94 => stringify!(KEY_MUHENKAN),
95 => stringify!(KEY_KPJPCOMMA),
96 => stringify!(KEY_KPENTER),
97 => stringify!(KEY_RIGHTCTRL),
98 => stringify!(KEY_KPSLASH),
99 => stringify!(KEY_SYSRQ),
100 => stringify!(KEY_RIGHTALT),
101 => stringify!(KEY_LINEFEED),
102 => stringify!(KEY_HOME),
103 => stringify!(KEY_UP),
104 => stringify!(KEY_PAGEUP),
105 => stringify!(KEY_LEFT),
106 => stringify!(KEY_RIGHT),
107 => stringify!(KEY_END),
108 => stringify!(KEY_DOWN),
109 => stringify!(KEY_PAGEDOWN),
110 => stringify!(KEY_INSERT),
111 => stringify!(KEY_DELETE),
112 => stringify!(KEY_MACRO),
113 => stringify!(KEY_MUTE),
114 => stringify!(KEY_VOLUMEDOWN),
115 => stringify!(KEY_VOLUMEUP),
// SC System Power Down
116=> stringify!(KEY_POWER),
117 => stringify!(KEY_KPEQUAL),
118 => stringify!(KEY_KPPLUSMINUS),
119 => stringify!(KEY_PAUSE),
// AL Compiz Scale (Expose)
120=> stringify!(KEY_SCALE),

121 => stringify!(KEY_KPCOMMA),
122 => stringify!(KEY_HANGEUL),
KEY_HANGUEL  => stringify!(KEY_HANGEUL),
123 => stringify!(KEY_HANJA),
124 => stringify!(KEY_YEN),
125 => stringify!(KEY_LEFTMETA),
126 => stringify!(KEY_RIGHTMETA),
127 => stringify!(KEY_COMPOSE),
// AC Stop 
128=> stringify!(KEY_STOP),
129 => stringify!(KEY_AGAIN),
// AC Properties
130 => stringify!(KEY_PROPS),
131/* AC Undo */ => stringify!(KEY_UNDO),
132 => stringify!(KEY_FRONT),
133/* AC Copy */ => stringify!(KEY_COPY),
134/* AC Open */ => stringify!(KEY_OPEN),
135/* AC Paste */ => stringify!(KEY_PASTE),
136/* AC Search */ => stringify!(KEY_FIND),
137/* AC Cut */ => stringify!(KEY_CUT	),
138/* AL Integrated Help Center */ => stringify!(KEY_HELP),
139/* Menu (show menu) */ => stringify!(KEY_MENU),
140/* AL Calculator */ => stringify!(KEY_CALC),
141 => stringify!(KEY_SETUP),
142/* SC System Sleep */ => stringify!(KEY_SLEEP),
143/* System Wake Up */ => stringify!(KEY_WAKEUP),
144/* AL Local Machine Browser */ => stringify!(KEY_FILE),
145 => stringify!(KEY_SENDFILE),
146 => stringify!(KEY_DELETEFILE),
147 => stringify!(KEY_XFER),
148 => stringify!(KEY_PROG1),
149 => stringify!(KEY_PROG2),
150/* AL Internet Browser */ => stringify!(KEY_WWW	),
151 => stringify!(KEY_MSDOS),
152/* AL Terminal Lock/Screensaver */ => stringify!(KEY_COFFEE),
KEY_SCREENLOCK=>	stringify!(KEY_COFFEE),
153/* Display orientation for e.g. tablets */ => stringify!(KEY_ROTATE_DISPLAY	),
KEY_DIRECTION=> stringify!(	KEY_ROTATE_DISPLAY),
154 => stringify!(KEY_CYCLEWINDOWS),
155 => stringify!(KEY_MAIL),
156/* AC Bookmarks */ => stringify!(KEY_BOOKMARKS),
157 => stringify!(KEY_COMPUTER),
158/* AC Back */ => stringify!(KEY_BACK),
159/* AC Forward */ => stringify!(KEY_FORWARD),
160 => stringify!(KEY_CLOSECD),
161 => stringify!(KEY_EJECTCD),
162 => stringify!(KEY_EJECTCLOSECD),
163 => stringify!(KEY_NEXTSONG),
164 => stringify!(KEY_PLAYPAUSE),
165 => stringify!(KEY_PREVIOUSSONG),
166 => stringify!(KEY_STOPCD),
167 => stringify!(KEY_RECORD),
168 => stringify!(KEY_REWIND),
169/* Media Select Telephone */ => stringify!(KEY_PHONE),
170 => stringify!(KEY_ISO),
171/* AL Consumer Control Configuration */ => stringify!(KEY_CONFIG),
172/* AC Home */ => stringify!(KEY_HOMEPAGE),
173/* AC Refresh */ => stringify!(KEY_REFRESH),
174/* AC Exit */ => stringify!(KEY_EXIT),
175 => stringify!(KEY_MOVE),
176 => stringify!(KEY_EDIT),
177 => stringify!(KEY_SCROLLUP),
178 => stringify!(KEY_SCROLLDOWN),
179 => stringify!(KEY_KPLEFTPAREN),
180 => stringify!(KEY_KPRIGHTPAREN),
181/* AC New */ => stringify!(KEY_NEW	),
182/* AC Redo/Repeat */ => stringify!(KEY_REDO),

183 => stringify!(KEY_F13),
184 => stringify!(KEY_F14),
185 => stringify!(KEY_F15),
186 => stringify!(KEY_F16),
187 => stringify!(KEY_F17),
188 => stringify!(KEY_F18),
189 => stringify!(KEY_F19),
190 => stringify!(KEY_F20),
191 => stringify!(KEY_F21),
192 => stringify!(KEY_F22),
193 => stringify!(KEY_F23),
194 => stringify!(KEY_F24),

200 => stringify!(KEY_PLAYCD),
201 => stringify!(KEY_PAUSECD),
202 => stringify!(KEY_PROG3),
203 => stringify!(KEY_PROG4),
204/* AC Desktop Show All Applications */ => stringify!(KEY_ALL_APPLICATIONS	),
KEY_DASHBOARD => stringify!(KEY_ALL_APPLICATIONS),
205 => stringify!(KEY_SUSPEND),
206/* AC Close */ => stringify!(KEY_CLOSE),
207 => stringify!(KEY_PLAY),
208 => stringify!(KEY_FASTFORWARD),
209 => stringify!(KEY_BASSBOOST),
210/* AC Print */ => stringify!(KEY_PRINT),
211 => stringify!(KEY_HP),
212 => stringify!(KEY_CAMERA),
213 => stringify!(KEY_SOUND),
214 => stringify!(KEY_QUESTION),
215 => stringify!(KEY_EMAIL),
216 => stringify!(KEY_CHAT),
217 => stringify!(KEY_SEARCH),
218 => stringify!(KEY_CONNECT),
219/* AL Checkbook/Finance */ => stringify!(KEY_FINANCE),
220 => stringify!(KEY_SPORT),
221 => stringify!(KEY_SHOP),
222 => stringify!(KEY_ALTERASE),
223/* AC Cancel */ => stringify!(KEY_CANCEL),
224 => stringify!(KEY_BRIGHTNESSDOWN),
225 => stringify!(KEY_BRIGHTNESSUP),
226 => stringify!(KEY_MEDIA),

    /* Cycle between available video 		   outputs (Monitor/LCD/TV-out/etc) */
227=> stringify!(KEY_SWITCHVIDEOMODE	),

228 => stringify!(KEY_KBDILLUMTOGGLE),
229 => stringify!(KEY_KBDILLUMDOWN),
230 => stringify!(KEY_KBDILLUMUP),

231/* AC Send */ => stringify!(KEY_SEND),
232/* AC Reply */ => stringify!(KEY_REPLY),
233/* AC Forward Msg */ => stringify!(KEY_FORWARDMAIL),
234/* AC Save */ => stringify!(KEY_SAVE),
235 => stringify!(KEY_DOCUMENTS),

236 => stringify!(KEY_BATTERY),

237 => stringify!(KEY_BLUETOOTH),
238 => stringify!(KEY_WLAN),
239 => stringify!(KEY_UWB),

240 => stringify!(KEY_UNKNOWN),

241/* drive next video source */ => stringify!(KEY_VIDEO_NEXT),
242/* drive previous video source */ => stringify!(KEY_VIDEO_PREV),
243/* brightness up, after max is min */ => stringify!(KEY_BRIGHTNESS_CYCLE	),
244=> stringify!(KEY_BRIGHTNESS_AUTO),
    /* Set Auto Brightness: manual
		  brightness control is off,
		  rely on ambient */
KEY_BRIGHTNESS_ZERO=> stringify!(KEY_BRIGHTNESS_AUTO),
245/* display device to off state */ => stringify!(KEY_DISPLAY_OFF),

246/* Wireless WAN (LTE, UMTS, GSM, etc.) */ => stringify!(KEY_WWAN),
KEY_WIMAX=> stringify!(	KEY_WWAN),
247/* Key that controls all radios */ => stringify!(KEY_RFKILL),

248/* Mute / unmute the microphone */ => stringify!(KEY_MICMUTE),

/* Code 255 is reserved for special needs of AT keyboard driver */
}
}

// TODO: this is incomplete
