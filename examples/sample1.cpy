      ******************************************************************
      *
      *    TODO:
      *    - COMP-1
      *
      ******************************************************************
       01  STRUCT.
           03 STRUCT-TEXTE                       PIC X(60) VALUE
           "Message de test".
           03 STRUCT-I                           PIC 9999.
           03 STRUCT-J                           PIC 9(6).
           03 STRUCT-J-REDEF REDEFINES STRUCT-J.
               05 FILLER                         PIC 99.
               05 STRUCT-JJJ                     PIC 9(4).
           03 STRUCT-BUFFER-1024                 PIC X(1024).
      *    03 STRUCT-RETURNCODE                  PIC S9(4) BINARY.
           03 STRUCT-NUMERIC-VALUE               PIC ZZZ'ZZZ.ZZ.
           03 STRUCT-NUMERIC-VALUE-9V9           PIC S9(6)V9(2).
      *-----------------------------------------------------------------
      *    UN COMMENTAIRE
      *-----------------------------------------------------------------
           03 STRUCT-ARRAY OCCURS 10.
               05 FILLER                         PIC XX.
               05 STRUCT-ARRAY-NO                PIC 99.
               05 STRUCT-ARRAY-NO-REF REDEFINES STRUCT-ARRAY-NO 
                                                 PIC XX.
               05 STRUCT-ARRAY-NOM               PIC X(100).
           03 STRUCT-NEXT                        PIC 9.
           03 STRUCT-OCCURS-INSIDE OCCURS 10     PIC 99.
       01  STRUCT2ERR.
           03 STRUCT2ERR-LONG-TEXT               PIC X(65000).
           03 STRUCT2ERR-LONG-TEXT-OCCURS-NEXT OCCURS 10.
           05 STRUCT2ERR-LONG-NEXT-TEXT2     PIC X(100).
