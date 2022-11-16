% Цей клас забезпечує оформлення робіт відповідно до вимог 
% ДСТУ 3008:2015 "ДОКУМЕНТАЦІЯ. ЗВІТИ У СФЕРІ НАУКИ І ТЕХНІКИ"
%
% Назва класу і його версія. Необхідна версія LaTeX
\ProvidesClass{khsu}[2017/05/18]
\NeedsTeXFormat{LaTeX2e}
%
% Базовий класс
\LoadClass[14pt,oneside]{extreport}
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\RequirePackage{polyglossia}
\setdefaultlanguage{ukrainian}
\setotherlanguages{english,russian}
\defaultfontfeatures{Ligatures={TeX},Renderer=Basic}

\setmainfont{Times New Roman} 
%hack for bibtex to work
\newfontfamily\cyrillicfonttt{Times New Roman}

\setsansfont{Arial}
\setmonofont{Courier New}


\usepackage{csquotes}

% for bibtex
\usepackage[backend=bibtex,style=numeric]{biblatex}


%
% Размерности листа
\RequirePackage[xetex,a4paper, top=2cm, bottom=2cm, left=2.5cm, right=1cm]{geometry}
% Починить отступ в первой строке абзаца
\RequirePackage{indentfirst}
% Гиперссылки в тексте
\usepackage[hidelinks,xetex]{hyperref}
% Поддержка цветов
\usepackage{xcolor}
% Поддержка графики
%\usepackage[pdftex]{graphicx}



%
\RequirePackage{lipsum}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Настройка колонтитулов, в т.ч. - номера страницы
\RequirePackage{fancyhdr}
\fancyhf{} % очистка текущих значений
\fancyhead[R]{\thepage} % установка верхнего колонтитула
\renewcommand{\headrulewidth}{0pt} % убрать разделительную линию
\pagestyle{fancy}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Абзацный отступ
\setlength{\parindent}{5ex}
\setlength{\parskip}{0pt}
% Интервал - полуторный
\linespread{1.3}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Глубина нумерации
\setcounter{secnumdepth}{3}
% Глубина содержания
\setcounter{tocdepth}{2}
% Управление заголовками
\RequirePackage[raggedright]{titlesec}
% Управление содержанием
\RequirePackage{titletoc}
%\titleformat{command}[shape]
%	{format}
%	{label}
%	{sep}
%	{before-code}
%	[after-code]
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%	РОЗДІЛ
\titleformat{\chapter}[display] % раздел
	{\filcenter}	% центровать
    {\MakeUppercase{\chaptertitlename} \thechapter}	% РОЗДІЛ Х 
    {0pt} % отступ после
	{\MakeUppercase}	% название капсом
	{}
\assignpagestyle{\chapter}{fancy}
\titlespacing{\chapter} % указуємо, що модифікуємо саме розділ
	{0ex} % відступ зліва
	{-30pt} % відступ зверху 
	{8pt} % відступ знизу
\titlecontents{chapter}
	[0ex] %
	{}
	{\bfseries{\chaptername\ \thecontentslabel {}\quad}\bfseries}
	{}
	{\dotfill\contentspage}    
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%	ПІДРОЗДІЛ
\titleformat{\section}[block]
    {\normalsize}
    {\thesection\quad}
    {1em}
    {}
\titlespacing{\section} % указуємо, що модифікуємо саме підрозділ
	{\parindent} % відступ зліва
	{0pt} % відступ зверху 
	{0pt} % відступ знизу 
\titlecontents{section}
	[5ex] %
	{}
	{\thecontentslabel\quad}
	{}
	{\dotfill\contentspage}    
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%	ПУНКТ
\titleformat{\subsection}[block]
    {\normalsize}
    {\thesubsection\quad}
    {1em}
    {}
\titlespacing{\subsection} % указуємо, що модифікуємо саме пункт
	{\parindent} % відступ зліва
	{12pt} % відступ зверху 
	{0pt} % відступ знизу 
\titlecontents{subsection}
	[10ex] %
	{}
	{\thecontentslabel {}\quad}
	{}
	{\dotfill\contentspage}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%	ПІДПУНКТ
\titleformat{\subsubsection}[block]
    {\normalsize}
    {\thesubsubsection\quad}
    {1em}
    {}
\titlespacing{\subsubsection} % указуємо, що модифікуємо саме пункт
	{\parindent} % відступ зліва
	{0pt} % відступ зверху 
	{0pt} % відступ знизу 
\titlecontents{subsubsection}
	[15ex] %
	{}
	{\thecontentslabel {}\quad}
	{}
	{\dotfill\contentspage}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% СТРУКТУРНА ЧАСТИНА % Вступ, Висновки і т.п.
\newcommand{\likechapterheading}[1]{ 
	\newpage
	\begin{center}
	\MakeUppercase{#1}	% Заголовок, капсом и по центру
	\end{center}
	\mbox{}\newline}
        
	\makeatletter
		\renewcommand{\@dotsep}{2}
		\newcommand{\l@likechapter}[2]{{\@dottedtocline{0}{0pt}{0pt}{\bfseries #1}{#2}}}
	\makeatother
    
	\newcommand{\likechapter}[1]{
		\likechapterheading{#1} % собственно название
		\addcontentsline{toc}{likechapter}{{#1}}	% и содержание
	}
	
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
\RequirePackage{longtable}
\RequirePackage{caption}
\DeclareCaptionLabelFormat{tablecap}{Таблиця #2} % формат за промовчанням "Табл." не влаштовує
\captionsetup[table]{
format=hang,
margin=0em,
indention=0em,
singlelinecheck=false, 
justification=justified, 
labelsep=endash, 
labelformat=tablecap, 
belowskip=18pt, 
aboveskip=3pt
} % формат для таблиць
% нове оточення для довгих таблиць
\renewenvironment{table}[4]{
% 1 - структура столбцов
% 2 - название
% 3 - ссылка
% 4 - оглавление
	\begingroup\centering
	\setlength{\LTpre}{-14pt}
	\setlength{\LTpost}{14pt}
	\setlength{\LTcapwidth}{0pt}
	\begin{longtable}{#1}
		\kill
		\caption{#2}\label{#3}\\
		#4
		\endfirsthead
		\kill
		\caption*{Продовження таблиці \thetable}\\
		#4
		\endhead
		\hline
		\endfoot
}
	{%
	\end{longtable}
	\endgroup
}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Комманды с параметрами
\RequirePackage{xkeyval}
% Условный оператор
\RequirePackage{ifthen}
% Титульная страница
\makeatletter

\define@cmdkey[CW]{TitlePage}{MinistryName}[Міністерство освіти і науки України]{}
\define@cmdkey[CW]{TitlePage}{UniversityName}[Херсонський державний університет]{}
\define@cmdkey[CW]{TitlePage}{FacultyName}[Факультет фізики, математики та інформатики]{}
\define@cmdkey[CW]{TitlePage}{CollegeOwnership}[Назва кафедри]{}
\define@cmdkey[CW]{TitlePage}{CityOfResearch}[Місто]{}
\define@cmdkey[CW]{TitlePage}{YearOfResearch}[рік]{}
%%
\define@cmdkey[CW]{TitlePage}{TitleKind}[Вид роботи]{}
\define@cmdkey[CW]{TitlePage}{TitileName}[Тема роботи]{}
\define@cmdkey[CW]{TitlePage}{StudentName}[ПІБ]{}
\define@cmdkey[CW]{TitlePage}{Discipline}[]{}
\define@cmdkey[CW]{TitlePage}{StudentSex}[male]{}
\define@cmdkey[CW]{TitlePage}{StudentGroup}[група]{}
\define@cmdkey[CW]{TitlePage}{StudentCourse}[курс]{}
% \define@cmdkey[CW]{TitlePage}{StudentSpeciality}[Спеціальність]{}
%%
\define@cmdkey[CW]{TitlePage}{SupervisorDegree}[професор]{}
\define@cmdkey[CW]{TitlePage}{SupervisorName}[ПІБ]{}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
\newcommand{\MakeTitlePage}[1]{
	\setkeys[CW]{TitlePage}{#1}
	\thispagestyle{empty}
%
	\begin{center}
		\par\noindent\cmdCW@TitlePage@MinistryName
		\par\noindent\cmdCW@TitlePage@UniversityName
		\par\noindent\cmdCW@TitlePage@FacultyName
		\par\noindent\cmdCW@TitlePage@CollegeOwnership
		\vfill		
		\par\noindent\textbf{\cmdCW@TitlePage@TitleKind}
		\par\noindent з дисципліни
		\par\noindent\cmdCW@TitlePage@Discipline
		\par\noindent<<\cmdCW@TitlePage@TitileName>>
	\end{center}
%
	\vfill	
%
\begin{flushleft}
	\hyphenpenalty=10000 % не использовать переносы на титульной странице
	\pretolerance=10000
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
	\begin{minipage}[t]{1\textwidth}
		\begin{minipage}[t]{0.3\textwidth}
			\ifthenelse
				{\equal{\cmdCW@TitlePage@StudentSex}{male}}
				{\par\noindent Виконав:}
				{\par\noindent Виконала:}
		\end{minipage}
		\hfill
		\begin{minipage}[t]{0.5\textwidth}
			\par\noindent 
			\ifthenelse
				{\equal{\cmdCW@TitlePage@StudentSex}{male}}
				{студент}
				{студентка}
			\cmdCW@TitlePage@StudentCourse\ курсу \cmdCW@TitlePage@StudentGroup\ групи
			\par\noindent\cmdCW@TitlePage@StudentName
		\end{minipage}
	\end{minipage}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
\bigskip\par\bigskip
	\begin{minipage}[t]{1\textwidth}
		\begin{minipage}[t]{0.3\textwidth}
			\par\noindent Приняв:
		\end{minipage}
		\hfill
		\begin{minipage}[t]{0.5\textwidth}
			\par\noindent \cmdCW@TitlePage@SupervisorDegree
			\par\noindent \cmdCW@TitlePage@SupervisorName
		\end{minipage}
	\end{minipage}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
\end{flushleft}
\vfill
	\begin{center}
	\cmdCW@TitlePage@CityOfResearch\ \,--\, \cmdCW@TitlePage@YearOfResearch	
	\end{center}
	\pagebreak
}
\makeatother
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
